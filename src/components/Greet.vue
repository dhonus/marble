<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import {Howl, Howler} from 'howler';
import { appDataDir, join } from '@tauri-apps/api/path';
import { readBinaryFile } from "@tauri-apps/api/fs"

const greetMsg = ref("");
const name = ref("");

const selectedImagePath = ref("");

import { open } from "@tauri-apps/api/dialog"
import { convertFileSrc } from '@tauri-apps/api/tauri';

async function selectImage() {

  const selected = await open({
    multiple: false,
    filters: [{
      name: "Images",
      extensions: ["png", "webp", "jpg", "jpeg"]
    }]
  });

  if (selected !== null) {

    const path = convertFileSrc(selected);
    selectedImagePath.value = path;
    console.log(selectedImagePath)
  }
}


async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>

  <img :src="selectedImagePath" className="h-200"/>

  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
  <audio id="audio" controls>
  </audio>
</template>
