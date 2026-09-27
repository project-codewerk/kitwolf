import { reactive } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { isTauriAvailable } from './tauri.js'

// Mirror of the Rust-side session (see Snapshot in src-tauri/src/main.rs).
//   mode:     'idle' | 'menu' | 'recording' | 'replaying'
//   captured:     clicks captured so far while recording
//   capturedCtrl: how many of those were Ctrl+clicks
//   pending:      click count of a finished recording waiting for a name, or null
//   pendingCtrl:  how many of those were Ctrl+clicks
const state = reactive({
    mode: 'idle',
    kits: [],
    captured: 0,
    capturedCtrl: 0,
    pending: null,
    pendingCtrl: 0,
})

// "9 clicks" or "9 clicks · 2 ctrl"
const describeClicks = function(kit){
    const ctrl = kit.clicks.filter((click) => click.ctrl).length
    return kit.clicks.length + ' clicks' + (ctrl ? ' · ' + ctrl + ' ctrl' : '')
}

let connected = null

// Browser preview without Tauri: ?mode=menu | naming | recording (add &view=control for the control window).
const mockState = function(){
    const mode = new URLSearchParams(window.location.search).get('mode') || 'menu'
    return {
        mode: mode == 'naming' ? 'menu' : mode,
        kits: [
            { id: '1', name: 'Sniper + Ghillie', clicks: [...new Array(7).fill({ x: 0, y: 0, ctrl: false }), ...new Array(2).fill({ x: 0, y: 0, ctrl: true })] },
            { id: '2', name: 'Medic', clicks: new Array(6).fill({ x: 0, y: 0 }) },
            { id: '3', name: 'Fortification kit', clicks: new Array(11).fill({ x: 0, y: 0 }) },
        ],
        captured: 7,
        capturedCtrl: 2,
        pending: mode == 'naming' ? 7 : null,
        pendingCtrl: 0,
    }
}

const connect = function(){
    if(!connected){
        connected = (async () => {
            if(!isTauriAvailable()){
                Object.assign(state, mockState())
                return
            }
            await listen('state-changed', (event) => Object.assign(state, event.payload))
            Object.assign(state, await invoke('get_state'))
        })()
    }
    return connected
}

const call = function(command, args){
    if(!isTauriAvailable()){
        console.log('[kitwolf preview]', command, args)
        return Promise.resolve()
    }
    return invoke(command, args)
}

const commands = {
    closeMenu: () => call('close_menu'),
    startRecording: () => call('start_recording'),
    saveRecording: (name) => call('save_recording', { name }),
    discardRecording: () => call('discard_recording'),
    replayKit: (id) => call('replay_kit', { id }),
    deleteKit: (id) => call('delete_kit', { id }),
}

export { state, connect, commands, describeClicks }
