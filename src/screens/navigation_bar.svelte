<!--
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
        z-index: 99;
    }
 
    .navigation-option:hover {
        border-color: blue;
        cursor: pointer;
    }   

    .navigation-option a {
        color: black;
        z-index: 99;
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
        z-index: 99;
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
        z-index: 99;
    }

    .dropdown-option:hover {
        border-color: blue;
    }
</style>
-->
<div class="body">
    <ul class="navigation-bar">
        <li class="navigation-option" id="plain">File</li>
        <li class="navigation-option" id="plain">Options</li>
        <li class="navigation-option" id="dropdown">
            <div class="dropdown-text">
                Tools
            </div>
            <ul class="dropdown-content">
                <li class="dropdown-option">Sub one</li>
                <li class="dropdown-option">Sub two</li>
                <li class="dropdown-option">Sub three</li>
            </ul>
        </li> 
    </ul>
</div>

<style>
    .body {
        background-color: whitesmoke;
        cursor: default;
    }

    .navigation-bar { 
        overflow: hidden;
    }


    .navigation-bar #plain.navigation-option{
        float: left;
        font-size: 16px;
        color: black;
        text-decoration: none;
        text-align: left;
        padding: 5px;
        padding-top: 6px;
        padding-bottom: 2px; 
        border-bottom: 2px solid whitesmoke;
    }

    #dropdown.navigation-option {
        float: left;
        overflow: hidden;
    }

    #dropdown.navigation-option .dropdown-text {
        font-size: 16px;  
        border: none;
        outline: none;
        color: black;
        background-color: inherit;
        font-family: inherit;
        margin: 0;
        padding: 5px;
        padding-top: 6px;
        padding-bottom: 2px; 
        border-bottom: 2px solid whitesmoke;
    }

    #plain.navigation-option:hover, #dropdown.navigation-option:hover .dropdown-text {
        border-color: blue;
    }

    .dropdown-content {
        display: none;
        position: absolute;
        min-width:  80px;
        background-color: whitesmoke;
        box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
        z-index: 1;
    }

    .dropdown-content li {
        color: black;
        text-decoration: none;
        display: block;
        text-align: left;
        border-right: 2px solid whitesmoke;
    }

    .dropdown-content li:hover {
        border-color: blue;
    }

    #dropdown.navigation-option:hover .dropdown-content {
        display: block;
    }

    .dropdown-option {
        padding-left: 2px;
        margin: 4px;
    }
</style>