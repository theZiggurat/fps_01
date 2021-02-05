import { Engine, Scene } from '@babylonjs/core'
import React, { useEffect, useRef } from 'react'
import './SceneComponent.css';
import FPSCamera from '../../game/input.js';

export default (props) => {
    const reactCanvas = useRef(null);
    const { antialias, engineOptions, adaptToDeviceRatio, sceneOptions, onRender, onSceneReady, ...rest } = props;
    useEffect(() => {
        if (reactCanvas.current) {
            const engine = new Engine(reactCanvas.current, antialias, engineOptions, adaptToDeviceRatio);
            const scene = new Scene(engine, sceneOptions);
            if (scene.isReady()) {
                props.onSceneReady(scene)
            } else {
                scene.onReadyObservable.addOnce(scene => props.onSceneReady(scene));
            }
            engine.runRenderLoop(() => {
                if (typeof onRender === 'function') {
                    onRender(scene, camera);
                }
                scene.render();
            })
            const resize = () => {
                scene.getEngine().resize();
            }
            if (window) {
                window.addEventListener('resize', resize);
            }
            var camera = new FPSCamera(scene);
            return () => {
                scene.getEngine().dispose();
                if (window) {
                    window.removeEventListener('resize', resize);
                }
            }
        }
    }, [reactCanvas])
    return (
        <canvas className="renderCanvas" ref={reactCanvas} {...rest} />
    );
}