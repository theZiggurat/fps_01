import {KeyboardEventTypes} from '@babylonjs/core';

export const keys = {
    'forward': 87,
    'backward': 83,
    'left': 65,
    'right': 68,
    'jump': 32
}



class FPSCamera {

    constructor(scene) {
        this._keys = [];

        this.processKeyDown = (key, code) => {
            this._keys[code] = true;
            console.log("KEYDOWN REGISTERED FOR KEY; ", key, " WITH KEYCODE; ", code);
        }

        this.processKeyUp = (key, code) => {
            this._keys[code] = false;
            console.log("KEYUP REGISTERED FOR KEY; ", key, " WITH KEYCODE; ", code);
        }

        this.keyboardEventHandler = (evtData) => {
            let evt = evtData.event;
            if(evt.repeat) return;
        
            switch(evtData.type) {
                case KeyboardEventTypes.KEYDOWN:
                    this.processKeyDown(evt.key, evt.keyCode);
                    break;
                case KeyboardEventTypes.KEYUP:
                    this.processKeyUp(evt.key, evt.keyCode);
                    break;
            }
        }

        scene.onKeyboardObservable.add(
            this.keyboardEventHandler, 
            KeyboardEventTypes.KEYDOWN + KeyboardEventTypes.KEYUP
        );
    }
    
    

    
    
    
    
    
    
}

export default FPSCamera;