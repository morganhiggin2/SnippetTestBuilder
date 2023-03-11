<!--<div class="body">
    <div class="link-container">
        <a class="link">File</a>
        <a class="link">Options</a>
        <a class="link">Edit</a>
       
    </div>
</div>-->

<div class="body">
    <ul class="navigation-bar">
      <li class="navigation-option"><a href="#">One</a></li>
      <li class="navigation-option"><a href="#">Two</a>
        <ul class="dropdown">
          <li class="dropdown-option"><a href="#">Sub-1</a></li>
          <li class="dropdown-option"><a href="#">Sub-2</a></li>
          <li class="dropdown-option"><a href="#">Sub-3</a></li>
        </ul>
      </li>
      <li class="navigation-option"><a href="#">Three</a></li>
    </ul>
</div> 

<style>
    .body {
        background-color: whitesmoke;
        height: 100%;
    }

    .navigation-option {
        display: block;
        position: relative;
        text-decoration: none;
        text-align: left;
        padding: 5px;
        padding-top: 6px;
        padding-bottom: 2px; 
        float: left;
        border-bottom: 2px solid whitesmoke; 
    }
 
    .navigation-option:hover {
        border-color: blue;
        cursor: pointer;
    }   

    .navigation-option a {
        color: black;
    }

    .dropdown {
        background-color: whitesmoke;
        visibility: hidden;
        position: absolute;
        padding-left: 2px;
        margin-top: 4px;
        margin-left: -5px;
        opacity: 0;
        min-width: 80px;
    }    
        
    .navigation-option:hover > ul,

    .dropdown:hover {
        visibility: visible;
        opacity: 1;
        display: block;
    }

    .dropdown-option {
        clear: both;
        width: 100%;
        padding-top: 2px;
        padding-bottom: 2px;
        border-right: 2px solid whitesmoke;
    }

    .dropdown-option:hover {
        border-color: blue;
    }
</style>