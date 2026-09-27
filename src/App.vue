<script>
import Overlay from './components/Overlay.vue'
import Control from './components/Control.vue'
import { getWindowLabel } from './helpers/tauri.js'

export default {
	components: { Overlay, Control },
	data() {
		return {
			label: getWindowLabel()
		}
	}
}
</script>

<template>
	<Control v-if="label == 'control'"/>
	<Overlay v-else/>
</template>
