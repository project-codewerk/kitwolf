<!-- Shown while recording. The overlay is click-through here, so this is display only. -->
<template>
	<div class="rec-hud">
		<span class="rec-hud__dot"></span>
		<span class="rec-hud__label">Recording kit</span>
		<span class="rec-hud__count wd-mono">{{ String(state.captured).padStart(2, '0') }} clicks</span>
		<span class="rec-hud__count rec-hud__count--ctrl wd-mono">{{ String(state.capturedCtrl).padStart(2, '0') }} ctrl</span>
		<span class="rec-hud__hint"><span class="wd-kbd">F8</span> Finish</span>
	</div>
</template>

<style scoped>
.rec-hud {
	position: absolute;
	bottom: var(--wd-hud-inset);
	left: 50%;
	transform: translateX(-50%);
	display: flex;
	align-items: center;
	gap: 14px;
	padding: 10px 18px;
	background: var(--wd-black);
	border: 1px solid rgba(195, 55, 44, .5);
	border-top: var(--wd-border-heavy) solid var(--wd-alert-500);
	box-shadow: var(--wd-shadow-plate-lg);
	clip-path: polygon(10px 0, 100% 0, 100% calc(100% - 10px), calc(100% - 10px) 100%, 0 100%, 0 10px);
	pointer-events: none;
}
.rec-hud__dot {
	width: 10px;
	height: 10px;
	background: var(--wd-alert-500);
	animation: rec-blink 1.2s steps(1, end) infinite;
}
.rec-hud__label {
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	font-size: var(--wd-size-label);
	letter-spacing: var(--wd-track-label);
	text-transform: uppercase;
	color: var(--text-display);
}
.rec-hud__count {
	font-weight: var(--wd-weight-semibold);
	text-transform: uppercase;
	color: var(--wd-gold-500);
}
.rec-hud__count--ctrl {
	color: var(--text-muted);
}
.rec-hud__hint {
	display: flex;
	align-items: center;
	gap: 6px;
	padding-left: 14px;
	border-left: var(--wd-rule);
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	font-size: var(--wd-size-overline);
	letter-spacing: var(--wd-track-overline);
	text-transform: uppercase;
	color: var(--text-muted);
}
@keyframes rec-blink {
	50% { opacity: .25; }
}
</style>

<script>
import { state } from '../helpers/kitwolf.js';

export default {
	data() {
		return { state }
	}
}
</script>
