import logo from './logo.svg';
import './App.css';
import React from 'react';
import SceneComponent from './components/SceneComponent/SceneComponent';
import { Scene, Vector3, HemisphericLight, MeshBuilder,Color3, UniversalCamera } from '@babylonjs/core';
import keys from './game/input.js';

const engineOptions = {

};

class App extends React.Component {

  

  onSceneReady = (scene) => {
    const camera = new UniversalCamera("camera", new Vector3(0,0,-10), scene);

    const box = MeshBuilder.CreateBox("box", scene);
  } 

  onRender = (scene, camera) => {
    if(camera._keys[keys['jump']]) {
      console.log("jumped");
    }
  }
  
  render() {
    return (
      <div className="App">
        <header className="App-header">
          <SceneComponent 
            onSceneReady={this.onSceneReady}
            onRender={this.onRender}
            antialias={true}
            />
          
          
        </header>
      </div>
    );
  }
 
}

export default App;
