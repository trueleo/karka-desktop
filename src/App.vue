<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import HelloWorld from './components/HelloWorld.vue'
import Task from './components/Task.vue';
import Day from './components/Day.vue';
import Action from './components/Action.vue';
import Attribute from './components/Attribute.vue';
import Sidebar from './components/Sidebar.vue';

class ActionClass {
    constructor(action, argument, summary) {
        this.action = action
        this.argument = argument
        this.summary = summary
    }
}

let actionlist=['browser','wine','game', 'run', 'notify', 'command'];
let attributelist = [
  {type: "from", value: "05:40"},
  {type: "until", value: "08:30"},
  {type: "except", value: "friday"},
  {type: "on", value: "sunday"},
  {type: "next", value: "wednesday"},
  {type: "signal", value: "class"},
]

let tasks = [
  {
    actions: [
      {
        action: "browser",
        argument: "meet.google.com",
        summary: "daily meeting"
      },
      {
        action: "wine",
        argument: "pinball",
        summary: ""
      },
      {
        action: "command",
        argument: "pkill node",
        summary: "kill node test server"
      }
    ],
    attributes: [
      {type: "from", value: "05:40"},
      {type: "until", value: "07:00"},
      {type: "except", value: "thursday"}
    ]
  },
  {
    actions: [
      {
        action: "browser",
        argument: "reddit.com",
        summary: ""
      },
      {
        action: "browser",
        argument: "twitch.tv",
        summary: "twitch time"
      },

    ],
    attributes: [
      {type: "from", value: "09:30"},
      {type: "on", value: "saturday"}
    ]
  },
  // {
  //   actions: [
  //     {
  //       action: "browser",
  //       argument: "meet.google.com",
  //       summary: "daily meeting"
  //     },
  //     {
  //       action: "wine",
  //       argument: "pinball",
  //       summary: ""
  //     },
  //     {
  //       action: "command",
  //       argument: "pkill node",
  //       summary: "kill node test server"
  //     }
  //   ],
  //   attributes: [
  //     {type: "from", value: "05:40"},
  //     {type: "until", value: "07:00"},
  //     {type: "except", value: "thursday"}
  //   ]
  // },
]

</script>

<template>
<div class="flex">
  <div class="basis-[16rem] flex items-center">
    <h1 class="font-bold text-xl place-self-center ml-6"> KARKA </h1>
  </div>
  <Day class="border-b-2 flex-grow"/>
</div>

<div class="flex gap-x-6 pt-6">
  <Sidebar 
  :actions="actionlist"
  :attributes="attributelist"
  class="basis-[16rem]"
  />

  <div class="space-y-4">
    
    <Task v-for="task in tasks" v-bind="task" />

  </div>
</div>
</template>

<style>

* {
  color: white
}

html, body, #app {
  height: 100%;
  width: 100%;
  background-color: rgb(30 41 59);
}

#app {
  margin: auto;
  max-width: 900px;
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  padding: 0 3%;
}

</style>
