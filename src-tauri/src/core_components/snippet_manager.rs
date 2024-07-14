use crate::{state_management::{ApplicationState, window_manager::WindowSession, external_snippet_manager::{ExternalSnippet}, visual_snippet_component_manager::{self, VisualSnippetComponentManager}}, utils::sequential_id_generator::{SequentialIdGenerator, self}};
use crate::state_management::visual_snippet_component_manager::{FrontSnippetContent, FrontPipelineConnectorContent, FrontPipelineContent};
use std::{collections::HashMap, ops::Add, sync::MutexGuard};
use crate::utils::sequential_id_generator::Uuid;
use petgraph::{self, adj::EdgeIndex, data::{Build, DataMap, DataMapMut}, visit::{EdgeIndexable, EdgeRef}};

/// the manager of the snippets, and their links
pub struct SnippetManager {
    //list of uuid of snippets to index in edge adj list
    //edge adj list for snippets
    //list of components 
    snippets: HashMap<Uuid, SnippetComponent>,
    pipelines: HashMap<Uuid, PipelineComponent>,

    //mapping for pipeline connects to pipeline components
    pipeline_connector_to_pipeline: HashMap<Uuid, Uuid>,
    pipeline_connectors_to_snippet: HashMap<Uuid, Uuid>,

    // graph for keeping track of cycles
    // where the weight of the node is the uuid of the snippet it represents
    // and the edge weight is the number of connections (pipelines) form that snippet to the other snippet
    snippet_graph: petgraph::Graph<(), i16, petgraph::Directed>

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
    pipeline_connectors: Vec<PipelineConnectorComponent> 
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

impl Default for SnippetManager {
    fn default() -> Self {
        return SnippetManager {
            snippets: HashMap::with_capacity(12),
            pipelines: HashMap::with_capacity(12),
            pipeline_connector_to_pipeline: HashMap::with_capacity(24),
            pipeline_connectors_to_snippet: HashMap::with_capacity(24),
            snippet_graph: petgraph::Graph::new()
        };
    }
}

impl SnippetManager {
    /// create a new snippet
    pub fn new_snippet(&mut self, sequential_id_generator: &mut SequentialIdGenerator, external_snippet: &ExternalSnippet) -> Uuid {
        let pipeline_connectors = external_snippet.create_pipeline_connectors_for_io_points(sequential_id_generator);

        //call handler method, return value
        //return uuid of snippet
        return self.new_snippet_handler(sequential_id_generator, pipeline_connectors, external_snippet.get_uuid(), external_snippet.get_name());
    }

    fn new_snippet_handler(&mut self, sequential_id_generator: &mut SequentialIdGenerator, pipeline_connectors: Vec<PipelineConnectorComponent>, external_snippet_uuid: Uuid, external_snippet_name: String) -> Uuid {
         //add snippet to graph
        let graph_uuid = self.snippet_graph.add_node(());

        //create snippet component
        let mut snippet_component : SnippetComponent = SnippetComponent::new(graph_uuid, sequential_id_generator);

        //get snippet uuid before borrowed mut
        let snippet_uuid : Uuid = snippet_component.uuid;

        //get snippet name
        let snippet_name : String = external_snippet_name;

        //add components from external snippet to snippet
        snippet_component.external_snippet_uuid = external_snippet_uuid;
        snippet_component.name = snippet_name;

        //add pipeline connector uuid to snippet mapping
        for pipeline_connector in pipeline_connectors.iter() {
            self.pipeline_connectors_to_snippet.insert(pipeline_connector.get_uuid(), snippet_uuid);
        }

        //move pipeline connectors to snippet component
        snippet_component.pipeline_connectors = pipeline_connectors;

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

        //remove snippet from pipeline connector mapping
        for pipeline_connector_uuid in pipeline_connectors_uuid.into_iter() {
            match self.pipeline_connectors_to_snippet.remove(&pipeline_connector_uuid) {
                Some(_) => (),
                None => {
                    return Err("pipeline connector does not exist in mapping in snippet manager");
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
    
    /// find pipeline connector uuid from pipeipe uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector coming out of pipeline
    pub fn find_pipeline_uuid_from_pipeline_connector(&self, uuid: &Uuid) -> Option<Uuid> {
        //find uuid of pipeline connector
        return self.pipeline_connector_to_pipeline.get(uuid).cloned(); 
    }
    /// find snippet uuid from pipeline uuid
    /// 
    /// # Arguments
    /// * 'uuid' - uuid of the pipeline connector 
    pub fn find_snippet_uuid_from_pipeline_connector(&self, uuid: &Uuid) -> Option<Uuid> {
        return self.pipeline_connectors_to_snippet.get(uuid).cloned();
    }

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
        self.pipeline_connector_to_pipeline.insert(from_uuid, pipeline_uuid);
        self.pipeline_connector_to_pipeline.insert(to_uuid, pipeline_uuid);

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

        //delete front from pipeline connector to pipeline relationship 
        match self.pipeline_connector_to_pipeline.remove(&from_pipeline_connector_uuid) {
            Some(_) => (),
            None => {
                return Err("front from pipeline connector does not exist in pipeline connector to pipeline relationship");
            }
        };
 
        //delete front to pipeline connector to pipeline relationship 
        match self.pipeline_connector_to_pipeline.remove(&to_pipeline_connector_uuid) {
            Some(_) => (),
            None => {
                return Err("front to pipeline connector does not exist in pipeline connector to pipeline relationship");
            }
        };       

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
            let from_result = match self.find_pipeline_uuid_from_pipeline_connector(&from_uuid) {
                Some(_) => true,
                None => false
            };

            let to_result = match self.find_pipeline_uuid_from_pipeline_connector(&to_uuid) {
                Some(_) => true,
                None => false
            };

            if from_result || to_result {
                return Ok(false);
            }
        }

        //verify that one is an output and one is an input
        //TODO validate schemas
        {
            if from_pipeline_connector.get_input() == to_pipeline_connector.get_input() {
                return Ok(false);
            }
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


    pub fn check_pipeline_connector_capacity_full(&self, pipeline_connector_uuid: &Uuid) -> bool {
        //check if in pipeline connector map
        return match self.find_pipeline_uuid_from_pipeline_connector(pipeline_connector_uuid) {
            Some(_) => true,
            None => false 
        };
    }
}

impl SnippetComponent {
    pub fn new(graph_uuid: petgraph::prelude::NodeIndex, sequential_id_generator: &mut SequentialIdGenerator) -> Self {
        return SnippetComponent {
            uuid: sequential_id_generator.get_id(),
            graph_uuid: graph_uuid,
            name: String::new(),
            external_snippet_uuid: 0,
            pipeline_connectors: Vec::new()
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

    pub fn get_uuid(&self) -> Uuid {
        return self.uuid;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    /// get snippets as front snippet content
    pub fn get_snippet_to_front_snippet(&self, visual_snippet_component_manager: &mut VisualSnippetComponentManager, sequential_id_generator: &mut SequentialIdGenerator, snippet_manager: &SnippetManager) -> FrontSnippetContent {
        //get pipeline connectors as front pipeline connectors
        let front_pipeline_connectors = self.get_pipeline_connectors_as_pipeline_connector_content(visual_snippet_component_manager, sequential_id_generator);

        //create front snippet
        let front_snippet = FrontSnippetContent::new(
            visual_snippet_component_manager,
            sequential_id_generator.get_id(),
            self.get_name(),
            self.get_uuid(),
            front_pipeline_connectors
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
            )
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

}

impl PipelineConnectorComponent {
    pub fn new(sequential_id_generator: &mut SequentialIdGenerator, external_pipeline_connector_uuid: Uuid, name: &str, input: bool) -> Self {
        return PipelineConnectorComponent {
            uuid: sequential_id_generator.get_id(),
            external_pipeline_connector_uuid: external_pipeline_connector_uuid,
            name: name.clone().to_string(),
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

    use super::*;

    /// simple new snippet test
    #[test]
    fn test_new_snippet() {
        // create self, default
        let mut snippet_manager = SnippetManager::default(); 
        let mut sequential_id_generator = SequentialIdGenerator::default();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();

        // temp variable
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_pipeline_connector_one", true));

        let external_snippet_uuid = sequential_id_generator.get_id();
        let snippet_id = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, external_snippet_uuid, "testing_snippet".to_string());

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
        
        assert_eq!(snippet.pipeline_connectors.len(), 1);
        
        // assert that the pipeline connectors are added correctly
        assert_eq!(snippet.pipeline_connectors.get(0).unwrap().uuid, 1);
        // assert the name of the pipeline connector is correct
        assert_eq!(snippet.pipeline_connectors.get(0).unwrap().input, true);
        // assert that the name is the same
        assert_eq!(snippet.pipeline_connectors.get(0).unwrap().name, "input_pipeline_connector_one");
    }

    #[test]
    fn delete_snippet() {
        // create self, default
        let mut snippet_manager = SnippetManager::default(); 
        let mut sequential_id_generator = SequentialIdGenerator::default();
        let mut pipeline_connectors = Vec::<PipelineConnectorComponent>::new();

        // temp variable
        let external_pipeline_connector_uuid = sequential_id_generator.get_id();
        pipeline_connectors.push(PipelineConnectorComponent::new(&mut sequential_id_generator, external_pipeline_connector_uuid, "input_pipeline_connector_one", true));

        let external_snippet_uuid = sequential_id_generator.get_id();
        let snippet_uuid = snippet_manager.new_snippet_handler(&mut sequential_id_generator, pipeline_connectors, external_snippet_uuid, "testing_snippet".to_string());

        //TODO add more pipelines, add other snippets, make sure all pipelines are disconnected

        // delete snippet
        snippet_manager.delete_snippet(&snippet_uuid).unwrap();

        
    }
}
