<template>
    <CommandWindow />
    <FileList /><FileList />
</template>

<script>
import CommandWindow from './components/CommandWindow.vue'
import FileList from './components/FileList.vue'
//import {invoke} from '@tauri-apps/api/tauri'


  // hack to remove the white flashing
  //setTimeout(() => {invoke('show_app')}, 100)



// Single key
// hotkeys('a,command+r', function (event) {
//     //event.srcElement: input 
//     //event.target: input
//     if (event.target === "input") {
//         alert('you pressed a!')
//     }
//     alert('you pressed a!')
//     event.preventDefault()
// });




// hotkeys('*', function() {
//   if (hotkeys.shift) {
//     console.log('shift is pressed!');
//   }

//   if (hotkeys.ctrl) {
//     console.log('ctrl is pressed!');
//   }

//   if (hotkeys.alt) {
//     console.log('alt is pressed!');
//   }

//   if (hotkeys.option) {
//     console.log('option is pressed!');
//   }

//   if (hotkeys.control) {
//     console.log('control is pressed!');
//   }

//   if (hotkeys.cmd) {
//     console.log('cmd is pressed!');
//   }

//   if (hotkeys.command) {
//     console.log('command is pressed!');
//   }
//   console.log(hotkeys.getPressedKeyCodes())
// });



  export default {
      name: 'App',
    components: {
      CommandWindow,
      FileList,
    },



    
}
</script>


  <style>
    #app {
      font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  background-color: #282828;
  margin-top: 20px;
}
body {
  background-color: #282828;
  
}

</style>
