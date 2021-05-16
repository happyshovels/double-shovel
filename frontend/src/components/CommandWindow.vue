<template>
  <div v-bind:class="{ hidden: !showCommandWindow }" class="command-window">
    <input class="command-input" ref="commandinput" v-model="commandInput" @keyup.enter="onSubmitCommand" @keyup.prevent.esc="onCancelCommand"/>
  </div>
</template>

<script>

export default {
  name: 'CommandWindow',
  data() {
    return {
      text: '',
      commandInput: ''
    }
  },
  watch: {
    showCommandWindow: function(visible) {
      console.log(visible)
      if (visible) {
        console.log('focus!')
        setTimeout( () => { this.$refs.commandinput.focus() }, 50)
        
        this.$refs.commandinput.focus()
      }
    },
    count: function() {
      this.$refs.commandinput.focus()
    }
  },
  computed: {
    showCommandWindow () {
      return this.$store.state.showCommandWindow
    },
    count () {
      return this.$store.state.count
    }

  }, 
  methods: {
    onSubmitCommand: function() {
      this.$store.dispatch('executeCommand', this.commandInput)
      this.$store.dispatch('toggleCommandWindow')
      this.commandInput = ''
      this.$refs.commandinput.blur()
      

    },
    onCancelCommand: function(event) {
      this.$store.dispatch('toggleCommandWindow')
      this.commandInput = ''
      this.$refs.commandinput.blur()
      event.preventDefault()
    },
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

.hidden {
  visibility: hidden;
}

div.command-window {
  
  
  color: #ebdbb2;
  border-color: green;
  border: solid;
  margin-bottom: 1ex;
  
  /*position:absolute;
  top: 123px;*/
  background-color: #3c3836;
  margin: 0 auto 0 auto;
  position: fixed;

  top: 20%;
  left: 50%;
  /*margin-top: -50px;
  margin-left: 10;*/
  font-size: 3ex;
  width: 30ex;
  margin-left: -15ex; 

  
}

input.command-input {
  color: #ebdbb2;
  font-weight: bold;
  font-size: 2ex;
  background-color: #3c3836 ;
  /*border: 1px solid #50494599;*/
  border: none;
  border-radius: 3px 3px 3px 3px ;
  /*font-size: 4ex;*/
  margin: 1ex;
  width: 26ex;
}

input.command-input:focus {
  outline: none;
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
