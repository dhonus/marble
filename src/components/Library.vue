<template>
<div v-if="!selectedBook">
	<h1>Library</h1>
	<!-- let's do a grid of book shaped divs, each has a large capital letter of the title[0] and then the title and author at the bottom -->
	<div class="book-grid">
	<div v-for="book in library" class="book" @click="showBookDetail(book)">
		<div class="book-capital">{{ book.title[0] }}</div>
		<span class="metadata">
		<div class="book-title">{{ book.title }}</div>
		&nbsp;-&nbsp;
		<div class="book-author">{{ book.author }}</div>
		</span>
	</div>
	</div>
</div>
<div v-if="selectedBook" class="detail-modal">
	<div class="modal-content">
	<h2>{{ selectedBook.title }}</h2>
	<p>{{ selectedBook.author }}</p>
	<!-- chapters -->
	<div class="chapters">
		<div v-for="chapter in selectedBook.chapters" class="chapter" @click="playChapter(chapter.file)">
			{{ chapter.title }}
			{{ chapter.length }}s
		</div>
	</div>
	<button @click="closeBookDetail">Close</button>
	</div>
</div>
</template>

<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { homeDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/dialog";
import { convertFileSrc } from "@tauri-apps/api/tauri";

// the library is a prop
export default {
	props: ["library"],
	data() {
		return {
			selectedBook: null,
		};
	},
	methods: {
		showBookDetail(book) {
			this.selectedBook = book;
		},
		closeBookDetail() {
			this.selectedBook = null;
		},
		playChapter(file) {
			this.$emit("play-chapter", file);
		},
	},
};
</script>