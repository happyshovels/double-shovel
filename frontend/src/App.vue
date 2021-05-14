<template>
    <FileList /><FileList />
</template>

  <script>
    import FileList from './components/FileList.vue'





  export default {
      name: 'App',
    components: {
  //    HelloWorld,
      FileList,
    }
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
