<template>
  <div class="file-list">
    <input class="current-folder" v-model="path" placeholder="edit me" />
    <ul class="file-directory">
        <li class="file-name" @click="file_list_click(file, 'parent')">..</li>
        <li class="file-name" @click="file_list_click(file, 'folder')" v-for="file in files" :key="file">
        {{ file }}
        </li>
    </ul>
    <button @click="update_files">update</button>
  </div>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri'

import path from 'path'

export default {
  name: 'FileList',
  data() {
    return {
      path: '/Users/lukas/projects' ,
      files: [],
      timer: null,
      clicks: 0,
      double_click_delay: 500,
    }
  },
  computed: {
  }, 
  methods: {
    rust_call: function () {
      // `this` inside methods points to the Vue instance
      this.counter = this.counter + 1
      invoke('my_command', { message: 'I got ' + this.counter }).then((answer) => { 
        //console.log('Completed ' + answer)
        this.info = answer
        //alert(answer)
      })

    },


    say_hello: function() {
      //console.log('hello')
      invoke('my_command', { message: 'I got ' + this.counter }).then((answer) => { 
        //console.log('Completed ' + answer)
        this.info = answer
        //alert(answer)
      })
    },
    update_files: function() {
      invoke('get_folder_content', {path: this.path}).then((response) => { 
        console.log(response.files)
        this.files = response.files
      })
    },

    file_list_click: function(file, type) {
      this.clicks++;
      if (this.clicks === 1) {
        this.timer = setTimeout( () => {
          alert('click: ' + file)
          this.clicks = 0
        }, this.double_click_delay);
      } else {
          clearTimeout(this.timer);  
          this.clicks = 0;
          switch(type) {
            case 'folder': this.enter_folder(file); break;
            case 'parent': this.go_to_parent(); break;

            default: alert('this should never happen - type: ' + type); break;
          }
          
      }
    },

    enter_folder(file) {
      this.path = path.join(this.path, file)
      this.update_files()
    },

    go_to_parent() {
      this.path = path.dirname(this.path)
      this.update_files()
    }
  },
  mounted: function () {
    setInterval(function () {
      this.update_files();
    }.bind(this), 1000);  
  }
  
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}

a {
  color: #42b983;
}

div.file-list {
  border: none;
  color: black;
  text-align: left;
  width: 48%;
  display: inline-block;
}

input.current-folder {
  width: 90%;
  height: 3ex;
}

ul.file-directory {
    border: solid;
    border-width: 0.1ex;
    text-align: left;
    width: 90%;
    height: 400px;
    padding-left: 0.5ex;
}



li.file-name {
    color: black;
    margin-top: 0.5ex;
    margin-bottom: 0.5ex;
    margin-left: 0.5ex;
    background-color: #eee;
    list-style-type: none;
    cursor: default;
}

li.file-name:hover {
  background-color: #ddd;
}
</style>
