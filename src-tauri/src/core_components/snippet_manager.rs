use crate::{state_management::{external_snippet_manager::{ExternalSnippet, ExternalSnippetParameterType}, visual_snippet_component_manager::{self, FrontParameterContent, VisualSnippetComponentManager}, window_manager::WindowSession, ApplicationState}, utils::sequential_id_generator::{self, SequentialIdGenerator}};
use crate::state_management::visual_snippet_component_manager::{FrontSnippetContent, FrontPipelineConnectorContent, FrontPipelineContent};
use std::{collections::{HashMap, HashSet}, hash::Hash, ops::Add, sync::MutexGuard};
use crate::utils::sequential_id_generator::Uuid;
use bimap::BiHashMap;
use petgraph::{self, adj::EdgeIndex, data::{Build, DataMap, DataMapMut}, graph::{Node, NodeIndex}, visit::{EdgeIndexable, EdgeRef}};
use pyo3::{IntoPy, Py, PyAny};

/// the manager of the snippets, and their links
pub struct SnippetManager {
    //list of uuid of snippets to index in edge adj list
    //edge adj list for snippets
    //list of components 
    snippets: HashMap<Uuid, SnippetComponent>,
    pipelines: HashMap<Uuid, PipelineComponent>,

    //mapping for pipeline connects to pipeline components
    // TODO this fails for multiple pipielinse for the same pipeline connector
    
    //what we need is a bi searchable structure that allows us to get multiple results when calling pipeline connector to pipeline
    // and pipieline to pipeline connector
    pipeline_connector_to_pipeline: HashMap<Uuid, HashSet<Uuid>>,
    pipeline_connector_to_snippet: HashMap<Uuid, Uuid>,
    parameter_to_snippet: HashMap<Uuid, Uuid>,
    
    //TODO delete these thoughts
    // we use it to 
    // a) get pipeline from pipeline connector
    // b) get snippet from pipeline connector
    // now for the building of a map
    // c) we need to get all pipeline connectors for each pipeline
    // b) get each snippet for each pipeline connector
    // the problem is there can exists multiple pipeline connectors for each snippet, and multiple pipelines connectors for each pipeline
    //
    // we can do c in an efficient O(n) iterative processs
    // b needs to change to allow multiple pipelines for a pipeline connector

    // graph for keeping track of cycles
    // where the edge weight is the number of connections (pipelines) form that snippet to the other snippet
    snippet_graph: petgraph::stable_graph::StableGraph<(), i16, petgraph::Directed>,
    snippet_to_node_index: BiHashMap<Uuid, NodeIndex>

    //mapping for snippets and pipelines
    //list of uuid of snippets to index in edge adj list
    //uuid_to_edge_adj_index: HashMap<u32, usize>
    //edge adj list
}

//TODO to adapt for multi input and outputs
//have hashmap of <(from_uuid, to_uuid), pipeline_uuid>
//and bihashmap of <from_uuid, to_uuid> which only includes pipeline connectors involed in pipelines

/// the actual snippet itself
pub struct SnippetComponent {
    uuid: Uuid,
    graph_uuid: petgraph::graph::NodeIndex,
    name: String,
    external_snippet_uuid: Uuid,
    pipeline_connectors: Vec<PipelineConnectorComponent>,
    parameters: Vec<SnippetParameterComponent>
}

pub struct PipelineConnectorComponent {
    uuid: Uuid,
    external_pipeline_connector_uuid: Uuid,
    name: String,
    input: bool
}

pub struct PipelineComponent {
    uuid: Uuid, 
    graph_uuid: petgraph::graph::EdgeIndex,
    from_pipeline_connector_uuid: Uuid,
    to_pipeline_connector_uuid: Uuid
}

#[derive(Clone)]
pub struct SnippetParameterComponent {
    uuid: Uuid,
    name: String,
    content: SnippetParameterBaseStorage,
    p_type: ExternalSnippetParameterType 
}

#[derive(Clone)]
pub enum SnippetParameterBaseStorage {
    String(String)
}

impl IntoPy<Py::<PyAny>> for SnippetParameterBaseStorage {
    fn into_py(self, py: pyo3::Python<'_>) -> Py::<PyAny> {
        match self {
            SnippetParameterBaseStorage::String(val) => {
                return val.into_py(py);
            },
        }
    }
}

impl Default for SnippetManager {
    fn default() -> Self {
        return SnippetManager {
            snippets: HashMap::with_capacity(12),
            pipelines: HashMap::with_capacity(12),
            pipeline_connector_to_pipeline: HashMap::with_capacity(24),
            pipeline_connector_to_snippet: HashMap::with_capacity(24),
            parameter_to_snippet: HashMap::new(),
            snippet_graph: petgraph::stable_graph::StableGraph::new(),
            snippet_to_node_index: BiHashMap::new()
        };
    }
}

impl SnippetManager {
    /// create a new snippet
    pub fn new_snippet(&mut self, sequential_id_generator: &mut SequentialIdGenerator, external_snippet: &ExternalSnippet) -> Uuid {
        let pipeline_connectors = external_snippet.create_pipeline_connectors_for_io_points(sequential_id_generator);
        // create parameter components
        let parameters= external_snippet.create_parameter_components_for_parameters(sequential_id_generator);

        //call handler method, return value
        //return uuid of snippet
        return self.new_snippet_handler(sequential_id_generator, pipeline_connectors, parameters, external_snippet.get_uuid(), external_snippet.get_name());
    }

    fn new_snippet_handler(&mut self, sequential_id_generator: &mut SequentialIdGenerator, pipeline_connectors: Vec<PipelineConnectorComponent>, parameters: Vec::<SnippetParameterComponent>, external_snippet_uuid: Uuid, external_snippet_name: String) -> Uuid {
         //add snippet to graph
        let graph_uuid = self.snippet_graph.add_node(());

        //create snippet component
        let mut snippet_component : SnippetComponent = SnippetComponent::new(graph_uuid, sequential_id_generator);

        //get snippet uuid before borrowed mut
        let snippet_uuid : Uuid = snippet_component.uuid;

        // add to mapping
        self.snippet_to_node_index.insert(snippet_uuid, graph_uuid);

        //get snippet name
        let snippet_name : String = external_snippet_name;

        //add components from external snippet to snippet
        snippet_component.external_snippet_uuid = external_snippet_uuid;
        snippet_component.name = snippet_name;

        //add pipeline connector uuid to snippet mapping
        for pipeline_connector in pipeline_connectors.iter() {
            self.pipeline_connector_to_snippet.insert(pipeline_connector.get_uuid(), snippet_uuid);
        }

        //move pipeline connectors to snippet component
        snippet_component.pipeline_connectors = pipeline_connectors;

        //add parameters to parameter to snippet mapping
        for parameter in parameters.iter() {
            self.parameter_to_snippet.insert(parameter.uuid, snippet_uuid);
        }

        //add parameters
        snippet_component.parameters = parameters;


        //add to snippets list in snippet manager
        self.snippets.insert(snippet_uuid, snippet_component);

        //return uuid of snippet
        return snippet_uuid;

    }

    /// delete snippet component and it's pipeline connector components
    /// assumes all pipelines associated with this have been disconnected / removed
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the snippet
    pub fn delete_snippet(&mut self, uuid: &Uuid) -> Result<(), &'static str> {
        //find snippet
        let snippet_component = match self.find_snippet(uuid) {
            Some(result) => result,
            None => {
                return Err("snippet component with snippet uuid does not exist in snippet manager");
            }
        };

        let graph_uuid = snippet_component.graph_uuid;

        //get pipelines connectors
        let pipeline_connectors_uuid : Vec<Uuid> = (&snippet_component.pipeline_connectors)
            .into_iter()
            .map(|pc| -> Uuid {
                pc.uuid
            }
        )
            .collect();

        // get parameter uuids
        let parameter_uuids : Vec<Uuid> = (&snippet_component.parameters)
            .into_iter()
            .map(|pc| -> Uuid {
                pc.uuid
            }
        )
            .collect();

        //remove snippet from pipeline connector mapping
        for pipeline_connector_uuid in pipeline_connectors_uuid.into_iter() {
            match self.pipeline_connector_to_snippet.remove(&pipeline_connector_uuid) {
                Some(_) => (),
                None => {
                    return Err("pipeline connector does not exist in mapping in snippet manager");
                }
            };
        }
        
        //remove snippet from pipeline connector mapping
        for parameter_uuid in parameter_uuids.into_iter() {
            match self.parameter_to_snippet.remove(&parameter_uuid) {
                Some(_) => (),
                None => {
                    return Err("parameter does not exist in mapping in snippet manager");
                }
            };
        }

        //delete snippet
        match self.snippets.remove(uuid) {
            Some(_) => (),
            None => {
                return Err("snippet component with snippet uuid does not exist in mapping in snippet manager");
            }
        };

        //remove snippet and it's edges from the graph
        self.snippet_graph.remove_node(graph_uuid);

        // remove from mapping
        self.snippet_to_node_index.remove_by_left(uuid);

        return Ok(());
    }
    
    /// find snippet from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the snippet 
    pub fn find_snippet(&self, uuid: &Uuid) -> Option<&SnippetComponent>{
        //find pipeline in vector
        return self.snippets.get(uuid);
    }

    /// find mutable reference to snippet from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the snippet 
    pub fn find_snippet_mut(&mut self, uuid: &Uuid) -> Option<&mut SnippetComponent>{
        //find pipeline in vector
        return self.snippets.get_mut(uuid);
    }


    /// get reference to snippets
    pub fn get_snippets_as_ref(&self) -> Vec<&SnippetComponent> {
        return self.snippets.values().collect::<Vec<&SnippetComponent>>();
    }

    /// find snippet from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the snippet 
    pub fn find_pipeline(&self, uuid: &Uuid) -> Option<&PipelineComponent>{
        //find pipeline in vector
        return self.pipelines.get(uuid);
    }

    /// find mutable reference to snippet from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the snippet 
    pub fn find_pipeline_mut(&mut self, uuid: &Uuid) -> Option<&mut PipelineComponent>{
        //find pipeline in vector
        return self.pipelines.get_mut(uuid);
    }

    /// find snippet paramaeter component from uuid
    /// 
    /// # Arguments
    /// * 'uuid" - uuid of the parameter component
    pub fn find_parameter(&mut self, uuid: &Uuid) -> Option<&mut SnippetParameterComponent> {
        // find snippet from parameter to snippet lookup
        let snippet_uuid = self.parameter_to_snippet.get(uuid)?.to_owned();

        // get parameter
        let snippet = self.snippets.get_mut(&snippet_uuid)?;

        // find index of position
        let mut snippet_index = None;

        for (i, parameter_component) in snippet.parameters.iter().enumerate() {
            if parameter_component.uuid == *uuid {
                snippet_index = Some(i);
            }
        }

        // if none were found
        match snippet_index {
            Some(i) => {
                return Some(snippet.parameters.get_mut(i).unwrap());
            }
            None => {
                return None;
            }
        }
    }
    
    /// find pipeline connector uuid from pipeipe uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector coming out of pipeline
    pub fn find_pipeline_uuids_from_pipeline_connector(&self, uuid: &Uuid) -> Vec<Uuid> {
        //find uuid of pipeline connector
        return match self.pipeline_connector_to_pipeline.get(uuid) {
            Some(some) => some.to_owned().into_iter().collect(),
            None => Vec::new(),
        };
    }
    /// find snippet uuid from pipeline uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector 
    pub fn find_snippet_uuid_from_pipeline_connector(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.pipeline_connector_to_snippet.get(uuid).cloned();
    }

    /// get a deep copy of the internal graph
    /// with the weight of each node being the uuid of the snippet
    pub fn get_snippet_graph(&self) -> petgraph::stable_graph::StableGraph<Uuid, (), petgraph::Directed> {
        let mut new_graph = petgraph::stable_graph::StableGraph::<Uuid, (), petgraph::Directed>::new();

        // map of old graph node to new graph node
        let mut old_to_new_node = HashMap::<NodeIndex, NodeIndex>::new();

        // for each node
        for node_index in self.snippet_graph.node_indices() {
            // get snippet uuid of node
            // if this unwrap fails, there is a critical logic error in the code
            let snippet_uuid = self.snippet_to_node_index.get_by_right(&node_index).unwrap();

            // create new node with node weight as snippet uuid
            let new_node_index = new_graph.add_node(snippet_uuid.to_owned());

            // insert to mapping
            old_to_new_node.insert(node_index, new_node_index);
        }

        // for each edge
        for edge_index in self.snippet_graph.edge_indices() {
            // get connecting node indexes
            let (node_left_index, node_right_index) = self.snippet_graph.edge_endpoints(edge_index).unwrap();

            // get new node indexes
            let new_node_left_index = old_to_new_node.get(&node_left_index).unwrap().to_owned();
            let new_node_right_index = old_to_new_node.get(&node_right_index).unwrap().to_owned();

            // create edge in new graph
            new_graph.add_edge(new_node_left_index, new_node_right_index, ());
        }

        return new_graph;
    }

    //TODO tests check for multiple pipielines to connect
    /// create pipeline
    /// 
    /// # Arguments
    /// * 'from_uuid' from pipeline connector's uuid
    /// * 'to uuid' to pipeline connector's uuid
    pub fn create_pipeline(&mut self,  sequential_id_generator: &mut SequentialIdGenerator, from_uuid: Uuid, to_uuid: Uuid) -> Result<Uuid, &'static str> {
        //get valid direction of pipeline, as from_uuid and to_uuid are not guarnteed to be input:false -> input:true
        let mut from_uuid = from_uuid;
        let mut to_uuid = to_uuid;

        {
            let from_snippet_uuid = match self.find_snippet_uuid_from_pipeline_connector(&from_uuid) {
                Some(result) => result,
                None => {
                    return Err("from snippet uuid from pipeline connector not found");
                }
            };

            let from_snippet = match self.find_snippet(&from_snippet_uuid) {
                Some(result) => result,
                None => {
                    return Err("from snippet from pipeline connector not found")
                }
            };

            //verify pipeline connector exists in pipeline
            let from_pipeline_connector = match from_snippet.find_pipeline_connector(from_uuid) {
                Some(result) => result,
                None => {
                    return Err("from pipeline connector does not exist in snippet");
                }
            };

            //find pipeline connector uuid
            let to_snippet_uuid = match self.find_snippet_uuid_from_pipeline_connector(&to_uuid) {
                Some(result) => result,
                None => {
                    return Err("to snippet uuid from pipeline connector not found");
                }
            };

            let to_snippet = match self.find_snippet(&to_snippet_uuid) {
                Some(result) => result,
                None => {
                    return Err("to snippet from pipeline connector not found")
                }
            };

            //verify pipeline connector exists in pipeline
            let to_pipeline_connector = match to_snippet.find_pipeline_connector(to_uuid) {
                Some(result) => result,
                None => {
                    return Err("to pipeline connector does not exist in snippet");
                }
            };

            //if it is flowing input -> output
            if from_pipeline_connector.get_input() && !to_pipeline_connector.get_input() {
                //revert order
                let temp = to_uuid;
                to_uuid = from_uuid;
                from_uuid = temp;
            }
        }
        
        //TODO better way
        let mut graph_uuid_container: Option<petgraph::prelude::EdgeIndex> = Option::None;

        {
            let from_snippet_uuid = match self.find_snippet_uuid_from_pipeline_connector(&from_uuid) {
                Some(result) => result,
                None => {
                    return Err("from snippet uuid from pipeline connector not found");
                }
            };

            let from_snippet = match self.find_snippet(&from_snippet_uuid) {
                Some(result) => result,
                None => {
                    return Err("from snippet from pipeline connector not found")
                }
            };


            //find pipeline connector uuid
            let to_snippet_uuid = match self.find_snippet_uuid_from_pipeline_connector(&to_uuid) {
                Some(result) => result,
                None => {
                    return Err("to snippet uuid from pipeline connector not found");
                }
            };

            let to_snippet = match self.find_snippet(&to_snippet_uuid) {
                Some(result) => result,
                None => {
                    return Err("to snippet from pipeline connector not found")
                }
            };

            // attempt to find existing edge
            let edge_result = self.snippet_graph.find_edge(from_snippet.graph_uuid, to_snippet.graph_uuid);

            graph_uuid_container = Some(match edge_result {
                Some(edge) => {
                    // get current weight
                    let edge_weight = self.snippet_graph.edge_weight_mut(edge).unwrap();

                    // update existing edge
                    *edge_weight = *edge_weight + 1;

                    // return edge index
                    edge
                },
                None => {
                    // create new edge, returning edge index
                    let edge_index = self.snippet_graph.add_edge(from_snippet.graph_uuid, to_snippet.graph_uuid, 1);

                    edge_index
                }
            });
        }

        //create new pipeline
        let pipeline_component = PipelineComponent::new(graph_uuid_container.unwrap(), sequential_id_generator, &from_uuid, &to_uuid);

        //get pipeline uuid for return
        let pipeline_uuid = pipeline_component.get_uuid();

        //add pipeline connecting values to snippet manager
        // append to list if inserting, else, create new
        match self.pipeline_connector_to_pipeline.get_mut(&from_uuid) {
            Some(uuids) => {
                // if it already contains it
                if uuids.contains(&pipeline_uuid) {
                    return Err("pipeline uuid already in pipeline connector to pipeline mapping");
                }

                uuids.insert(pipeline_uuid);
            },
            None => {
                let mut uuids = HashSet::<Uuid>::from([pipeline_uuid]);

                self.pipeline_connector_to_pipeline.insert(from_uuid, uuids);
            }
        }
        match self.pipeline_connector_to_pipeline.get_mut(&to_uuid) {
            Some(uuids) => {
                // if it already contains it
                if uuids.contains(&pipeline_uuid) {
                    return Err("pipeline uuid already in pipeline connector to pipeline mapping");
                }

                uuids.insert(pipeline_uuid);
            },
            None => {
                let mut uuids = HashSet::<Uuid>::from([pipeline_uuid]);

                self.pipeline_connector_to_pipeline.insert(to_uuid, uuids);
            }
        }

        //add to snippet manager
        self.pipelines.insert(pipeline_uuid, pipeline_component);

        //return uuid of new pipeline
        return Ok(pipeline_uuid);
    }

    /// deletes pipeline and all snippet manager internal links
    /// upon unsuccessfull deletion, deletes taken before in this method are not reversed
    /// 
    /// # Arguments 
    /// * 'uuid' - uuid of the pipeline component
    pub fn delete_pipeline(&mut self, uuid: &Uuid) -> Result<(), &'static str> {
        let mut from_pipeline_connector_uuid = 0;
        let mut to_pipeline_connector_uuid = 0;
        let mut graph_uuid_container: Option<petgraph::prelude::EdgeIndex> = Option::None;

        //get pipeline uuids without violating borrow rules for self
        {
            //get pipeline
            let pipeline_component = match self.find_pipeline(uuid) {
                Some(result) => result,
                None => {
                    return Err("pipeline does not exist in snippet manager to delete"); 
                }
            };

            //grab copies of the pipeline connector uuids
            from_pipeline_connector_uuid = pipeline_component.from_pipeline_connector_uuid.clone();
            to_pipeline_connector_uuid = pipeline_component.to_pipeline_connector_uuid.clone();
            graph_uuid_container = Some(pipeline_component.graph_uuid);
        }
        
        //get edge weight
        let edge_weight = self.snippet_graph.edge_weight_mut(graph_uuid_container.unwrap()).unwrap();

        // if there is more than one existing pipeline in this direction still connecting, then reduce weight
        if *edge_weight > 1 {
            *edge_weight = *edge_weight - 1;
        }
        // delete the edge
        else {
            self.snippet_graph.remove_edge(graph_uuid_container.unwrap());
        }

        // whether to delete the entry
        let mut delete_entry = false;

        //delete front from pipeline connector to pipeline relationship 
        match self.pipeline_connector_to_pipeline.get_mut(&from_pipeline_connector_uuid) {
            Some(uuids) => {
                // if uuids contains the pipeline connector, remove it
                let contained = uuids.remove(&uuid);

                // if it did not contain it
                if !contained {
                    return Err("pipeline uuid does not exist in the from pipeline connector to pipeline relationship inner set");
                }

                if uuids.len() == 0 {
                    delete_entry = true;
                }
            },
            None => {
                return Err("front from pipeline connector does not exist in pipeline connector to pipeline relationship");
            }
        };

        // if there are no more elements in the inner set of the mapping, delete it
        if delete_entry {
            self.pipeline_connector_to_pipeline.remove(&from_pipeline_connector_uuid);
        }

        // whether to delete the entry
        let mut delete_entry = false;

        //delete front from pipeline connector to pipeline relationship 
        match self.pipeline_connector_to_pipeline.get_mut(&to_pipeline_connector_uuid) {
            Some(uuids) => {
                // if uuids contains the pipeline connector, remove it
                let contained = uuids.remove(&uuid);

                // if it did not contain it
                if !contained {
                    return Err("pipeline uuid does not exist in to pipeline connector to pipeline relationship inner set");
                }

                if uuids.len() == 0 {
                    delete_entry = true;
                }
            },
            None => {
                return Err("to pipeline connector does not exist in pipeline connector to pipeline relationship");
            }
        };

        // if there are no more elements in the inner set of the mapping, delete it
        if delete_entry {
            self.pipeline_connector_to_pipeline.remove(&to_pipeline_connector_uuid);
        }

        //delete pipeline in snippet manager
        match self.pipelines.remove(uuid) {
            Some(_) => (),
            None => {
                return Err("pipeline does not exist in snippet manager");
            }
        }

        return Ok(());
    }

    /// validate pipeline
    /// returning weither or not this is a valid pipeline creation
    /// assumed by error that the pipeline connectors their
    /// underying snippets exist
    /// 
    /// # Arguments
    /// * 'from_uuid' from pipeline connector's uuid
    /// * 'to uuid' to pipeline connector's uuid
    pub fn validate_pipeline(&mut self, from_uuid: Uuid, to_uuid: Uuid) -> Result<bool, &'static str> {
        //find pipeline connectors
        //find pipeline connector uuid
        let from_snippet_uuid = match self.find_snippet_uuid_from_pipeline_connector(&from_uuid) {
            Some(result) => result,
            None => {
                return Err("from snippet uuid from pipeline connector not found");
            }
        };

        let from_snippet = match self.find_snippet(&from_snippet_uuid) {
            Some(result) => result,
            None => {
                return Err("from snippet from pipeline connector not found")
            }
        };

        //verify pipeline connector exists in pipeline
        let from_pipeline_connector = match from_snippet.find_pipeline_connector(from_uuid) {
            Some(result) => result,
            None => {
                return Err("from pipeline connector does not exist in snippet");
            }
        };

        //find pipeline connector uuid
        let to_snippet_uuid = match self.find_snippet_uuid_from_pipeline_connector(&to_uuid) {
            Some(result) => result,
            None => {
                return Err("to snippet uuid from pipeline connector not found");
            }
        };

        let to_snippet = match self.find_snippet(&to_snippet_uuid) {
            Some(result) => result,
            None => {
                return Err("to snippet from pipeline connector not found")
            }
        };

        //verify pipeline connector exists in pipeline
        let to_pipeline_connector = match to_snippet.find_pipeline_connector(to_uuid) {
            Some(result) => result,
            None => {
                return Err("to pipeline connector does not exist in snippet");
            }
        };

        {
            //verify that a connection between the two same does not already exist
            //TODO allow mulitple pipelines
            let from_result = self.find_pipeline_uuids_from_pipeline_connector(&from_uuid).len() > 0;

            let to_result = self.find_pipeline_uuids_from_pipeline_connector(&to_uuid).len() > 0;

            if from_result || to_result {
                return Ok(false);
            }
        }

        //verify that from is an output and to is an input
        //TODO validate schemas
        {
            /*if from_pipeline_connector.get_input() {
                return Ok(false);
            }

            if !to_pipeline_connector.get_input() {
                return Ok(false);
            }*/
        }

        //verify that the connection is between different snippets
        if from_snippet.get_uuid() == to_snippet.get_uuid() {
            return Ok(false);
        }

        // attempt to find existing edge
        let edge_result = self.snippet_graph.find_edge(from_snippet.graph_uuid, to_snippet.graph_uuid);

        let dag_valid = match edge_result {
            // We know this if valid from a dag standpoint because the connection already exists
            Some(_) => true,
            None => {
                // create new edge, returning edge index
                let edge_index = self.snippet_graph.add_edge(from_snippet.graph_uuid, to_snippet.graph_uuid, 1);

                // check for cycle, if there is one, remove edge and return nothing
                let mut is_dag = true; 

                if petgraph::algo::is_cyclic_directed(&self.snippet_graph) {
                    is_dag = false;
                }

                // remove edge
                self.snippet_graph.remove_edge(edge_index);

                is_dag
            }
        };

        // if we did not pass the dag check, return false
        if dag_valid == false {
            return Ok(false);
        }
        /*
        //verify types match
        if from_pipeline_connector.get_type() != to_pipeline_connector.get_type() {
            return Ok(false); 
        }*/

        return Ok(true);
    }


    /// check if we cannot accept any more pipeline connectors
    pub fn check_pipeline_connector_capacity_full(&self, pipeline_connector_uuid: &Uuid) -> bool {
        //check if in pipeline connector map
        // TODO allow multiple
        return self.find_pipeline_uuids_from_pipeline_connector(pipeline_connector_uuid).len() > 0;
    }

    /// Validate if the current snippet configuration is ready being being ran 
    /// i.e in valid run state
    pub fn validate_for_run(&self) -> bool {
        return true
    }

    /// Generate mapping of each from (snippet_uuid, output_name) -> [(snippet_uuid, input_name), ...]
    /// according to the pipeline connections 
    pub fn generate_snippet_io_point_mappings(&self) -> HashMap<(Uuid, String), Vec<(Uuid, String)>> {
        let mut map: HashMap<(Uuid, String), Vec<(Uuid, String)>> = HashMap::new();

        // for each pipeline component
        for (_pipeline_component_uuid, pipeline_component) in self.pipelines.iter() {
            // insert ((from_snippet_id, out_name), (to_snippet_id, input_nama) append this last one to the list)
            // get from connector uuid and to connector uuid
            let from_pipeline_connector_uuid = pipeline_component.from_pipeline_connector_uuid.clone();

            // get snippet
            // can unwrap safely here, if it fails, there is a critical code logic error
            let snippet_component_uuid = self.pipeline_connector_to_snippet.get(&from_pipeline_connector_uuid).unwrap();
            let snippet_component = self.snippets.get(snippet_component_uuid).unwrap();

            // set as from snippet uuid
            let from_snippet_uuid = snippet_component_uuid.clone();

            let mut from_name = None;

            // from snippets pipeline connectors, find the from pipeline connector
            for pipeline_connector in snippet_component.pipeline_connectors.iter() {
                if pipeline_connector.get_uuid().eq(&from_pipeline_connector_uuid) {
                    from_name = Some(pipeline_connector.get_name());
                }
            }

            // if we did not find it, we have a logic error
            if let None = from_name {
                panic!("Could not find from pipeline connector in corresponding snippet");
            }

            // get from connector uuid and to connector uuid
            let to_pipeline_connector_uuid = pipeline_component.to_pipeline_connector_uuid.clone();

            // get snippet
            // can unwrap safely here, if it fails, there is a critical code logic error
            let snippet_component_uuid = self.pipeline_connector_to_snippet.get(&to_pipeline_connector_uuid).unwrap();
            let snippet_component = self.snippets.get(snippet_component_uuid).unwrap();

            // set as from snippet uuid
            let to_snippet_uuid = snippet_component_uuid.clone();
            // from snippets pipeline connectors, find the from pipeline connector

            let mut to_name = None;

            // from snippets pipeline connectors, find the from pipeline connector
            for pipeline_connector in snippet_component.pipeline_connectors.iter() {
                if pipeline_connector.get_uuid().eq(&to_pipeline_connector_uuid) {
                    to_name = Some(pipeline_connector.get_name());
                }
            }

            // if we did not find it, we have a logic error
            if let None = to_name {
                panic!("Could not find to pipeline connector in corresponding snippet");
            }

            // if snippet uuid, from connector name exists
            match map.get_mut(&(from_snippet_uuid.to_owned(), from_name.to_owned().unwrap())) {
                Some(outputs) => {
                    // append (to snippet uuid, to connector name)
                    outputs.push((to_snippet_uuid, to_name.unwrap()));
                },
                None => {
                    // new vec with size one, containing (to snippet uuid, to connector name)
                    // for key (to snippet uuid, to connector name)
                    map.insert((from_snippet_uuid, from_name.unwrap()), Vec::from([(to_snippet_uuid, to_name.unwrap())]));
                },
            }
        }
        
        return map;
    }
}

impl SnippetComponent {
    pub fn new(graph_uuid: petgraph::prelude::NodeIndex, sequential_id_generator: &mut SequentialIdGenerator) -> Self {
        return SnippetComponent {
            uuid: sequential_id_generator.get_id(),
            graph_uuid: graph_uuid,
            name: String::new(),
            external_snippet_uuid: 0,
            pipeline_connectors: Vec::new(),
            parameters: Vec::new()
        }
    }

    /// find mutable refernece to pipeline connector from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector
    fn find_pipeline_connector(&self, uuid: Uuid) -> Option<&PipelineConnectorComponent>{
        //find pipeline in vector
        return self.pipeline_connectors.iter().find(|pipe: &&PipelineConnectorComponent| pipe.uuid == uuid);
    }

    /// find mutable refernece to pipeline connector from uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector
    fn find_pipeline_connector_mut(&mut self, uuid: Uuid) -> Option<&mut PipelineConnectorComponent>{
        //find pipeline in vector
        return self.pipeline_connectors.iter_mut().find(|pipe: &&mut PipelineConnectorComponent| pipe.uuid == uuid);
    }

    /// create deep copy of parameters
    pub fn get_parameters_as_copy(&self) -> Vec<SnippetParameterComponent> {
        return self.parameters.clone();
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_external_snippet_id(&self) -> Uuid {
        return self.external_snippet_uuid;
    }

    /// get snippets as front snippet content
    pub fn get_snippet_to_front_snippet(&self, visual_snippet_component_manager: &mut VisualSnippetComponentManager, sequential_id_generator: &mut SequentialIdGenerator, snippet_manager: &SnippetManager) -> FrontSnippetContent {
        //get pipeline connectors as front pipeline connectors
        let front_pipeline_connectors = self.get_pipeline_connectors_as_pipeline_connector_content(visual_snippet_component_manager, sequential_id_generator);

        //get parameters as front
        let front_parameters = self.get_parameters_as_parameter_content(visual_snippet_component_manager, sequential_id_generator);

        //create front snippet
        let front_snippet = FrontSnippetContent::new(
            visual_snippet_component_manager,
            sequential_id_generator.get_id(),
            self.get_name(),
            self.get_uuid(),
            front_pipeline_connectors,
            front_parameters
        );

        return front_snippet;
    }

    fn get_pipeline_connectors_as_pipeline_connector_content(&self, visual_snippet_component_manager: &mut VisualSnippetComponentManager, sequential_id_generator: &mut SequentialIdGenerator) -> Vec<FrontPipelineConnectorContent> {
        //generate contents vector
        let mut contents: Vec<FrontPipelineConnectorContent> = Vec::with_capacity(self.pipeline_connectors.len());

        //TODO try to find in visual snippet component manager by calling find_ method on visual snippet component manager, if not, then create using pipeline_connector.create_as_front_content
        // have this approach for many
        //get io points
        for pipeline_connector in self.pipeline_connectors.iter() {
            //push to contents
            contents.push(
                FrontPipelineConnectorContent::new(
                    visual_snippet_component_manager,
                    sequential_id_generator.get_id(),
                    pipeline_connector.uuid.clone(),
                    pipeline_connector.name.clone(),
                    pipeline_connector.input
                )
            );
        }

        return contents;
    }

    fn get_parameters_as_parameter_content(&self, visual_snippet_component_manager: &mut VisualSnippetComponentManager, sequential_id_generator: &mut SequentialIdGenerator) -> Vec<FrontParameterContent> {
        let mut contents: Vec<FrontParameterContent> = Vec::with_capacity(self.parameters.len());

        // build parameter fronts from parameters
        for parameter in self.parameters.iter() {
            // look up snippet parmater in external snippet parameters to get ptype
            contents.push(
                FrontParameterContent::new(
                    visual_snippet_component_manager,
                   sequential_id_generator.get_id(),
                   parameter.uuid,
                   parameter.name.to_owned(),
                   parameter.p_type.to_string()
                )
            );
        }

        return contents;
    }

    /// get list of uuids of the pipeline connectors
    /// associated with this pipeline
    pub fn get_pipeline_connector_uuids(&self) -> Vec<Uuid> {
        return (&self.pipeline_connectors).into_iter().map(|pcc| -> Uuid {
            return pcc.uuid;
        }).collect();
    }

    pub fn get_input_names(&self) -> Vec<String> {
        return self.pipeline_connectors.iter().filter(|connector| -> bool {connector.input == true}).map(|connector| -> String {connector.get_name()}).collect();
    }

    pub fn get_output_names(&self) -> Vec<String> {
        return self.pipeline_connectors.iter().filter(|connector| -> bool {connector.input == false}).map(|connector| -> String {connector.get_name()}).collect();
    }
}

impl PipelineConnectorComponent {
    pub fn new(sequential_id_generator: &mut SequentialIdGenerator, external_pipeline_connector_uuid: Uuid, name: &str, input: bool) -> Self {
        return PipelineConnectorComponent {
            uuid: sequential_id_generator.get_id(),
            external_pipeline_connector_uuid: external_pipeline_connector_uuid,
            name: name.to_string(),
            input: input
        }
    }

    pub fn get_name(&self) -> String {
        return self.name.to_owned();
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    pub fn get_input(&self) -> bool {
        return self.input;
    }
}

impl PipelineComponent {
    pub fn new(graph_uuid: petgraph::graph::EdgeIndex, sequential_id_generator: &mut SequentialIdGenerator, from_uuid: &Uuid, to_uuid: &Uuid) -> Self {
        return PipelineComponent {
            uuid: sequential_id_generator.get_id(),
            graph_uuid: graph_uuid,
            from_pipeline_connector_uuid: from_uuid.clone(),
            to_pipeline_connector_uuid: to_uuid.clone()
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    /// creates a new front content for this pipeline
    /// returns the front pipeline content 
    pub fn create_pipeline_as_front_content(&self, visual_snippet_component_manager: &mut VisualSnippetComponentManager, sequential_id_generator: &mut SequentialIdGenerator) -> FrontPipelineContent {
        let front_pipeline_content = FrontPipelineContent::new(visual_snippet_component_manager, sequential_id_generator.get_id(), self.uuid); 

        visual_snippet_component_manager.put_pipeline(front_pipeline_content.get_uuid(), self.uuid);

        return front_pipeline_content;
    }

    /// get front from pipeline connector 
    pub fn get_from_pipeline_connector_uuid(&self) -> Uuid {
        return self.from_pipeline_connector_uuid;
    }

    /// get front to pipeline connector 
    pub fn get_to_pipeline_connector_uuid(&self) -> Uuid {
        return self.to_pipeline_connector_uuid;
    }
}

impl SnippetParameterComponent {
    pub fn new(storage: SnippetParameterBaseStorage, name: String, p_type: ExternalSnippetParameterType, sequential_id_generator: &mut SequentialIdGenerator) -> Self {
        return SnippetParameterComponent {
            uuid: sequential_id_generator.get_id(),
            name: name,
            content: storage,
            p_type: p_type
        };
    }

    /// Update value of parameter, value will be given in string format
    /// and attempt to convert to its base type, failing on failure.
    pub fn update_value(&mut self, value: String) -> Result<(), &'static str> {
        self.content = match self.content {
            SnippetParameterBaseStorage::String(_) => {
                // attempt to convert to base type, which is string
                // since we are already a string, no strict conversion needed
                SnippetParameterBaseStorage::String(value)
            }
        };

        return Ok(());
    }
    
    pub fn get_name(&self) -> String {
        return self.name.to_owned();
    }

    pub fn get_storage(&self) -> &SnippetParameterBaseStorage {
        return &self.content;
    }

    pub fn get_p_type(&self) -> ExternalSnippetParameterType {
        return self.p_type.clone();
    }
}
//map: pipeline_connectors->parent

//could store parent uuid for connector inside connector or mapping
//TODO change alot of impl from getting self seperatrly to directory getting &self or &mut self
//TODO implement automatically the order of pipeline so that input: false always goes to input:true, also add validation for this to make sure
//  that they appose
//TODO visual grid, and gridlocking



//TODO rust backend visual to id mapper 
//TODO convert all then()'s to .await
//TODO hide all new shippet stuff with internal id's behind abstraction layer of virtual component manager
//TODO be able to move visual snippet components on screen, have pipelines either (a disapear temproarly, so visiabiliy off, then redraw), or if working
//  b) be able to have dotted lines that move with the snippet as the pipelines when it moves (can implement a first, then b)

#[cfg(test)]
mod tests {
    use crate::{core_components::snippet_manager::SnippetManager, utils::sequential_id_generator::SequentialIdGenerator};
    use crate::state_management::external_snippet_manager::IntoStorageType;

    use super::*;

    /// simple new snippet test
    #[test]
    fn test_new_snippet() {
        // create self, default
        let mut snippet_manager = SnippetManager::default(); 
        let mut sequential_id_generator = SequentialIdGenerator::default();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();

        // temp variable
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_pipeline_connector_one", true));

        let parameter_storage = ExternalSnippetParameterType::into_storage_type(&ExternalSnippetParameterType::SingleLineText); 
        let parameter = SnippetParameterComponent::new(parameter_storage, "param_one".to_string(), ExternalSnippetParameterType::SingleLineText, &mut sequential_id_generator);
        let parameter_uuid = parameter.uuid;
        parameters.push(parameter); 

        let external_snippet_uuid = sequential_id_generator.get_id();
        let snippet_id = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, external_snippet_uuid, "testing_snippet".to_string());

        // assert that the state of the snippet manager was changed in the correct way
        // assert that the snippet was added by the correct id
        let snippet_get_result = snippet_manager.snippets.get(&snippet_id);
        let snippet = match snippet_get_result {
            Some(snippet) => snippet,
            None => {
                assert!(false);

                return;
            },
        };

        // check contents of snippet
        assert_eq!(snippet.name, "testing_snippet");
        assert_eq!(snippet.external_snippet_uuid, external_snippet_uuid);
        
        // assert graph is assembled correctly
        let snippet_node_weight_result = snippet_manager.snippet_graph.node_weight(snippet.graph_uuid);

        let snippet_node_weight = match snippet_node_weight_result {
            Some(weight) => weight,
            None => {
                assert!(false);

                return;
            },
        };

        // weight is of empty type
        assert_eq!(*snippet_node_weight, ());
        // assert only one node in graph
        assert_eq!(snippet_manager.snippet_graph.node_count(), 1);
        // asssert no edges
        assert_eq!(snippet_manager.snippet_graph.edge_count(), 0);

        assert_eq!(snippet_manager.snippet_to_node_index.len(), 1);
        
        assert_eq!(snippet.pipeline_connectors.len(), 1);
        
        // assert that the pipeline connectors are added correctly
        assert_eq!(snippet.pipeline_connectors.get(0).unwrap().uuid, 1);
        // assert the name of the pipeline connector is correct
        assert_eq!(snippet.pipeline_connectors.get(0).unwrap().input, true);
        // assert that the name is the same
        assert_eq!(snippet.pipeline_connectors.get(0).unwrap().name, "input_pipeline_connector_one");
        
        // assert parmeters were inserted correctly
        assert_eq!(snippet.parameters.len(), 1);
        assert_eq!(snippet.parameters.get(0).unwrap().name, "param_one");
        assert_eq!(snippet.parameters.get(0).unwrap().uuid, parameter_uuid);
        assert_eq!(snippet.parameters.get(0).unwrap().p_type, ExternalSnippetParameterType::SingleLineText);
        /*match snippet.parameters.get(0).unwrap().content {

        }*/
    }

    #[test]
    fn delete_snippet() {
        // create self, default
        let mut snippet_manager = SnippetManager::default(); 
        let mut sequential_id_generator = SequentialIdGenerator::default();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();

        // create the first snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));

        let first_external_snippet_uuid = sequential_id_generator.get_id();
        let first_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, first_external_snippet_uuid, "testing_snippet_one".to_string());

        // create second snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_two", false));

        let second_external_snippet_uuid = sequential_id_generator.get_id();
        let second_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, second_external_snippet_uuid, "testing_snippet_two".to_string());


        // create third snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_two", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));

        let parameter_storage = ExternalSnippetParameterType::into_storage_type(&ExternalSnippetParameterType::SingleLineText); 
        let parameter = SnippetParameterComponent::new(parameter_storage, "param_one".to_string(), ExternalSnippetParameterType::SingleLineText, &mut sequential_id_generator);
        let parameter_uuid = parameter.uuid;
        parameters.push(parameter); 

        let third_external_snippet_uuid = sequential_id_generator.get_id();
        let third_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, third_external_snippet_uuid, "testing_snippet_one".to_string());


        // delete third snippet
        snippet_manager.delete_snippet(&third_snippet_uuid).unwrap();

        // check that there are no exsting pipelines, as all of them were connected to snippet three
        assert_eq!(snippet_manager.pipelines.len(), 0);
        assert_eq!(snippet_manager.pipeline_connector_to_pipeline.len(), 0);

        // check that there still exists the same number of snippets
        assert_eq!(snippet_manager.snippets.len(), 2);

        // check that there are the same number of pipeline connectors, minus the ones from three
        assert_eq!(snippet_manager.pipeline_connector_to_snippet.len(), 5);

        // make sure the pipeline connectors from the third one are 
        //TODO insert rest
        assert_eq!(snippet_manager.pipeline_connector_to_snippet.contains_key(&1), true);
        assert_eq!(snippet_manager.pipeline_connector_to_snippet.contains_key(&2), true);
        assert_eq!(snippet_manager.pipeline_connector_to_snippet.contains_key(&6), true);
        assert_eq!(snippet_manager.pipeline_connector_to_snippet.contains_key(&7), true);
        assert_eq!(snippet_manager.pipeline_connector_to_snippet.contains_key(&8), true);

        // assert only two nodes in graph
        assert_eq!(snippet_manager.snippet_graph.node_count(), 2);
        // asssert no edges 
        assert_eq!(snippet_manager.snippet_graph.edge_count(), 0);

        assert_eq!(snippet_manager.snippet_to_node_index.len(), 2);
    }

    #[test]
    fn test_create_pipeline() {
        // create self, default
        let mut snippet_manager = SnippetManager::default(); 
        let mut sequential_id_generator = SequentialIdGenerator::default();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();

        // create the first snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));

        let first_external_snippet_uuid = sequential_id_generator.get_id();
        let first_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, first_external_snippet_uuid, "testing_snippet_one".to_string());

        // create second snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_two", false));

        let second_external_snippet_uuid = sequential_id_generator.get_id();
        let second_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, second_external_snippet_uuid, "testing_snippet_two".to_string());

        // create third snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_two", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));

        let parameter_storage = ExternalSnippetParameterType::into_storage_type(&ExternalSnippetParameterType::SingleLineText); 
        let parameter = SnippetParameterComponent::new(parameter_storage, "param_one".to_string(), ExternalSnippetParameterType::SingleLineText, &mut sequential_id_generator);
        let parameter_uuid = parameter.uuid;
        parameters.push(parameter); 

        let third_external_snippet_uuid = sequential_id_generator.get_id();
        let third_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, third_external_snippet_uuid, "testing_snippet_one".to_string());

        // connect one to three
        snippet_manager.create_pipeline(&mut sequential_id_generator, 2, 12).unwrap();
        
        // connect three to one
        snippet_manager.create_pipeline(&mut sequential_id_generator, 7, 13).unwrap();

        // check that pipelines exists
        assert_eq!(snippet_manager.pipelines.contains_key(&15), false);
        assert_eq!(snippet_manager.pipelines.contains_key(&16), false);
        assert_eq!(snippet_manager.pipelines.len(), 2);

        // check that the snippets still exist
        assert_eq!(snippet_manager.snippets.len(), 3);
        assert_eq!(snippet_manager.snippets.contains_key(&4), true);
        assert_eq!(snippet_manager.snippets.contains_key(&10), true);
        assert_eq!(snippet_manager.snippets.contains_key(&17), true);

        // check that pipeline connects are good
        assert_eq!(snippet_manager.pipeline_connector_to_pipeline.len(), 4);
        assert_eq!(snippet_manager.pipeline_connector_to_snippet.len(), 8);
        assert_eq!(snippet_manager.parameter_to_snippet.len(), 1);

        // assert 3 nodes
        assert_eq!(snippet_manager.snippet_graph.node_count(), 3);

        // get snippet's graph uuid
        let snippet_one_graph_uuid = snippet_manager.find_snippet(&4).unwrap().graph_uuid;
        let snippet_two_graph_uuid = snippet_manager.find_snippet(&10).unwrap().graph_uuid;
        let snippet_three_graph_uuid = snippet_manager.find_snippet(&17).unwrap().graph_uuid;

        // asssert only two edges 
        assert_eq!(snippet_manager.snippet_graph.edge_count(), 2);

        assert!(match snippet_manager.snippet_graph.find_edge(snippet_one_graph_uuid, snippet_three_graph_uuid) {
            Some(_) => true,
            None => false,
        });
        assert!(match snippet_manager.snippet_graph.find_edge(snippet_two_graph_uuid, snippet_three_graph_uuid) {
            Some(_) => true,
            None => false,
        });

        let edge_one = snippet_manager.snippet_graph.find_edge(snippet_one_graph_uuid, snippet_three_graph_uuid).unwrap();
        let edge_two = snippet_manager.snippet_graph.find_edge(snippet_two_graph_uuid, snippet_three_graph_uuid).unwrap();

        assert!(match snippet_manager.snippet_graph.edge_weight(edge_one) {
            Some(val) => {
                if *val == 1 {
                   true 
                }
                else {
                    false
                }
            },
            None => false,
        });
        assert!(match snippet_manager.snippet_graph.edge_weight(edge_two) {
            Some(val) => {
                if *val == 1 {
                   true 
                }
                else {
                    false
                }
            },
            None => false,
        });
    }

    #[test]
    fn test_delete_pipeline() {
        // create self, default
        let mut snippet_manager = SnippetManager::default(); 
        let mut sequential_id_generator = SequentialIdGenerator::default();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();

        // create the first snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));

        let first_external_snippet_uuid = sequential_id_generator.get_id();
        let first_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, first_external_snippet_uuid, "testing_snippet_one".to_string());

        // create second snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_two", false));

        let second_external_snippet_uuid = sequential_id_generator.get_id();
        let second_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, second_external_snippet_uuid, "testing_snippet_two".to_string());


        // create third snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_two", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));

        let parameter_storage = ExternalSnippetParameterType::into_storage_type(&ExternalSnippetParameterType::SingleLineText); 
        let parameter = SnippetParameterComponent::new(parameter_storage, "param_one".to_string(), ExternalSnippetParameterType::SingleLineText, &mut sequential_id_generator);
        let parameter_uuid = parameter.uuid;
        parameters.push(parameter); 

        let third_external_snippet_uuid = sequential_id_generator.get_id();
        let third_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, third_external_snippet_uuid, "testing_snippet_one".to_string());


        // connect one to three
        snippet_manager.create_pipeline(&mut sequential_id_generator, 2, 12).unwrap();
        
        // connect two to three
        snippet_manager.create_pipeline(&mut sequential_id_generator, 7, 13).unwrap();
        
        // delete the pipeline going from one to three
        snippet_manager.delete_pipeline(&18).unwrap();

        // check that pipelines exists
        assert_eq!(snippet_manager.pipelines.contains_key(&15), false);
        assert_eq!(snippet_manager.pipelines.len(), 1);

        // check that the snippets still exist
        assert_eq!(snippet_manager.snippets.len(), 3);
        assert_eq!(snippet_manager.snippets.contains_key(&4), true);
        assert_eq!(snippet_manager.snippets.contains_key(&10), true);
        assert_eq!(snippet_manager.snippets.contains_key(&17), true);

        // check that pipeline connects are good
        assert_eq!(snippet_manager.pipeline_connector_to_pipeline.len(), 2);
        assert_eq!(snippet_manager.pipeline_connector_to_snippet.len(), 8);
        assert_eq!(snippet_manager.parameter_to_snippet.len(), 1);

        // assert 3 nodes
        assert_eq!(snippet_manager.snippet_graph.node_count(), 3);

        // get snippet's graph uuid
        let snippet_one_graph_uuid = snippet_manager.find_snippet(&4).unwrap().graph_uuid;
        let snippet_two_graph_uuid = snippet_manager.find_snippet(&10).unwrap().graph_uuid;
        let snippet_three_graph_uuid = snippet_manager.find_snippet(&17).unwrap().graph_uuid;

        // asssert only one edge 
        assert_eq!(snippet_manager.snippet_graph.edge_count(), 1);

        assert!(match snippet_manager.snippet_graph.find_edge(snippet_one_graph_uuid, snippet_three_graph_uuid) {
            Some(_) => false,
            None => true,
        });
        assert!(match snippet_manager.snippet_graph.find_edge(snippet_two_graph_uuid, snippet_three_graph_uuid) {
            Some(_) => true,
            None => false,
        });

        let edge_one = snippet_manager.snippet_graph.find_edge(snippet_two_graph_uuid, snippet_three_graph_uuid).unwrap();

        assert!(match snippet_manager.snippet_graph.edge_weight(edge_one) {
            Some(val) => {
                if *val == 1 {
                   true 
                }
                else {
                    false
                }
            },
            None => false,
        });
    }

    #[test]
    fn test_validate_pipeline() {
        // create self, default
        let mut snippet_manager = SnippetManager::default(); 
        let mut sequential_id_generator = SequentialIdGenerator::default();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();

        // create the first snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));

        let first_external_snippet_uuid = sequential_id_generator.get_id();
        let first_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, first_external_snippet_uuid, "testing_snippet_one".to_string());

        // create second snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_two", false));

        let second_external_snippet_uuid = sequential_id_generator.get_id();
        let second_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, second_external_snippet_uuid, "testing_snippet_two".to_string());


        // create third snippet
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();
        let mut parameters = Vec::<SnippetParameterComponent>::new();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_one", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_two", true));
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "output_one", false));

        let parameter_storage = ExternalSnippetParameterType::into_storage_type(&ExternalSnippetParameterType::SingleLineText); 
        let parameter = SnippetParameterComponent::new(parameter_storage, "param_one".to_string(), ExternalSnippetParameterType::SingleLineText, &mut sequential_id_generator);
        let parameter_uuid = parameter.uuid;
        parameters.push(parameter); 

        let third_external_snippet_uuid = sequential_id_generator.get_id();
        let third_snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, parameters, third_external_snippet_uuid, "testing_snippet_one".to_string());


        // connect one to three
        snippet_manager.create_pipeline(&mut sequential_id_generator, 2, 12).unwrap();

        // valid valid case (is dag, output to input)
        // validate two to three
        assert!(snippet_manager.validate_pipeline(7, 13).unwrap());

        // test if going to different snippets
        assert!(!snippet_manager.validate_pipeline(7, 8).unwrap());

        snippet_manager.create_pipeline(&mut sequential_id_generator, 7, 13).unwrap();

        // validate dag invalid case
        // validate three to one
        assert!(!snippet_manager.validate_pipeline(14, 6).unwrap());
    }
}
