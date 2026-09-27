<template>
	<div id="overlay">
		<KitMenu v-if="state.mode == 'menu'"/>
		<RecordingHud v-else-if="state.mode == 'recording'"/>
	</div>
</template>

<style scoped>
#overlay {
	position: fixed;
	inset: 0;
}
</style>

<script>
import KitMenu from './KitMenu.vue';
import RecordingHud from './RecordingHud.vue';
import { state, connect } from '../helpers/kitwolf.js';

export default {
	components: { KitMenu, RecordingHud },
	data() {
		return { state }
	},
	mounted: function(){
		connect();
	}
}
</script>
