import { createApp } from 'vue'
import { createStore } from 'vuex'

import App from './App.vue'
import hotkeys from 'hotkeys-js';

const store = createStore({
    state() {
        return {
            count: 0
        }
    },
    actions: {
        increment({ commit }) {
            commit('increment')
        },
        decrement({ commit }) {
            commit('decrement')
        }
    },

    mutations: {
        increment(state) {
            state.count++
        },
        decrement(state) {
            state.count--
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



createApp(App)
    .use(store)
    .mount('#app')
