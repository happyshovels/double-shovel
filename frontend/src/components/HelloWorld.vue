<template>
  <div class="hello">
    <h1 style="font-size:10ex">⭐</h1>
    <li v-for="file in files" :key="file">
      {{ file }}
    </li>
    <h1>{{ msg }}</h1>
    <p> {{ stars }} </p>
    <p>{{ info }}</p>
    <button v-on:click="rust_call">Blink</button>
    <button v-on:click="call_api">Api</button>
    <button v-on:click="call_state">State</button>
  </div>
</template>

<script>
   
    import {invoke} from '@tauri-apps/api/tauri'
    // With the Tauri global script:
    //const invoke = window.__TAURI__.invoke


export default {
  name: 'HelloWorld',
  data() {
    return {
      msg: 'Some App',
      info: '...',
      counter: 0 ,
      files: []
    }
  },
  computed: {
    stars: function() {
      return '*'.repeat(this.counter)
    }
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
    call_api: function () {
      invoke('plugin:awesome|do_something')
    },
    call_state: function () {
      invoke('check_my_state')
      //this.say_hello()
    },
    say_hello: function() {
      //console.log('hello')
      invoke('my_command', { message: 'I got ' + this.counter }).then((answer) => { 
        //console.log('Completed ' + answer)
        this.info = answer
        //alert(answer)
      })
    },
    log_files: function() {
      invoke('get_files').then((answer) => { 
        //console.log(answer)
        console.log(answer.files)
        this.files = answer.files
        // this.info = answer
        //alert(answer)
      })
    }
  },
  mounted: function () {
    //this.say_hello();

    // setInterval(function () {
    //   this.say_hello();
    // }.bind(this), 1000);

    setInterval(function () {
      this.log_files();
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
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
