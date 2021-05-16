import { createApp } from 'vue'
import { createStore } from 'vuex'

import App from './App.vue'
import hotkeys from 'hotkeys-js';

const store = createStore({
    state() {
        return {
            count: 0,
            showCommandWindow: false,
            lastCommand: '',
        }
    },
    actions: {
        increment({ commit }) {
            commit('increment')
        },
        decrement({ commit }) {
            commit('decrement')
        },
        toggleCommandWindow({ commit }) {
            commit('toggleCommandWindow')
        },
        executeCommand({ commit }, command) {
            commit('setLastCommand', command)
        }
    },

    mutations: {
        increment(state) {
            state.count++
        },
        decrement(state) {
            state.count--
        },
        toggleCommandWindow(state) {
            state.showCommandWindow = !state.showCommandWindow
        },
        setLastCommand(state, command) {
            state.lastCommand = command
        }
    }
})



// hotkeys
hotkeys('up', function (event) {
    store.dispatch('increment');
    event.preventDefault();
});

hotkeys('down', function (event) {
    store.dispatch('decrement');
    event.preventDefault();
});

hotkeys('command+shift+p', function (event) {
    store.dispatch('toggleCommandWindow');
    event.preventDefault();
});



createApp(App)
    .use(store)
    .mount('#app')
