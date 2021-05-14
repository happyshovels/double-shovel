<template>
  <div class="filelist">
    <input v-model="path" placeholder="edit me" />
    <br/>
    <br/>
    <ul class="file-directory">
        <li class="file-name" v-for="file in files" :key="file">
        {{ file }}
        </li>
    </ul>
    <br/>
    <button @click="update_files">update</button>
  </div>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri'

export default {
  name: 'FileList',
  data() {
    return {
      path: '/Users/lukas/projects' ,
      files: []
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
ul {
  list-style-type: none;
  padding: 0;
}
li {
  //display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}

ul.file-directory {
    border: solid;
    border-width: 0.1ex;
    text-align: left;
    width: 50%;
    height: 400px;
}



li.file-name {
    color: black;
    margin-top: 0.5ex;
    margin-bottom: 0.5ex;
    background-color: #eee;
    list-style-type: none;
    cursor: default;
}

li.file-name:hover {
  background-color: #ddd;
}
</style>
