<template>
  <div class="file-list">
    <input class="current-folder" v-model="path" placeholder="current directory" readonly />
    <ul class="file-directory">
        <li class="file-name dir" @click="file_list_click(file, 'parent')">..</li>
        <li class="file-name dir" @click="file_list_click(file.name, 'folder')" v-for="file in files.filter(f => f.entry_type == 'dir')" :key="file.name">
          {{ file.name }}
        </li>
        <li class="file-name" @click="file_list_click(file.name, 'file')" v-for="file in files.filter(f => f.entry_type == 'file')" :key="file.name">
          {{ file.name }}
        </li>
    </ul>
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
    update_files: function() {
      invoke('get_folder_content', {queryPath: this.path}).then((response) => { 
        //console.log(response.files)
        this.files = response.files
        //console.log(this.files)
      })
    },

    file_list_click: function(file, type) {
      this.clicks++;
      if (this.clicks === 1) {
        this.timer = setTimeout( () => {
          this.clicks = 0
        }, this.double_click_delay);
      } else {
          clearTimeout(this.timer);  
          this.clicks = 0;
          switch(type) {
            case 'folder': this.enter_folder(file); break;
            case 'parent': this.go_to_parent(); break;
            case 'file': this.open_file(path.join(this.path, file)); break;

            default: alert('this should never happen - type: ' + type); break;
          }
          
      }
    },

    enter_folder(file) {
      this.path = path.join(this.path, file)
      console.log(this.path)
      this.update_files()
    },

    go_to_parent() {
      this.path = path.dirname(this.path)
      this.update_files()
    },

    open_file(file_path) {
      invoke('open_file', {filePath: file_path}).then((response) => { 
        //console.log(response.files)
        console.log(response)
      })
    }
  },
  mounted: function () {
    this.update_files();
    setInterval(function () {
      this.update_files();
    }.bind(this), 1000);  

    //setTimeout(() => {invoke('close_splashscreen')}, 1000)

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
  background-color: #282828;
  color: #ebdbb2;
}

input.current-folder {
  /*
  width: 90%;
  height: 3ex;
  font-weight: bold;
  font-size: 2ex;
  color: #ebdbb2;
  background-color: #282828;

  box-shadow: none;
  border-radius: 0px;
  border-color: red;
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
  

  border: solid;
  box-shadow: none;
  */
  width: 88%;
  height: 3ex;
  color: #ebdbb2;
  font-weight: bold;
  font-size: 2ex;
  background-color: #282828 ;
    border: 1px solid #50494599 ;
    border-radius: 3px 3px 3px 3px ;
    padding-left: 1ex;
  /*etc.*/
}

ul.file-directory::-webkit-scrollbar {
  background: #50494599;
  width: 0.5ex;
}

ul.file-directory::-webkit-scrollbar-thumb {
  /*width: 30px;*/
  background-color: #eee;
  
}


ul.file-directory {
    border: solid;
    border-width: 0.1ex;
    text-align: left;
    width: 90%;
    height: 400px;
    padding-left: 0.5ex;
    user-select: none;
    overflow: scroll;
    overflow-x: hidden;
    border-color: #50494599;
    
}



li.file-name {
    color: black;
    margin-top: 0.5ex;
    margin-bottom: 0.5ex;
    margin-left: 0.5ex;
    /*background-color: #eee;*/
    list-style-type: none;
    cursor: default;
    color: #ebdbb2
}

li.file-name:hover {
  background-color: #3c3836;

}

li.dir {
  font-weight: bold;
  color: #689d6a;
}
</style>
