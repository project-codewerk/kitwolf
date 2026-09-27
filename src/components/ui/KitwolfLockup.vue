<!-- KITWOLF lockup from template/brand/Kitdog Logo.html (1B): wolf mark + wordmark. -->
<template>
	<div class="lockup">
		<img class="lockup__mark" :src="mark" alt="" :style="{ width: size + 'px', height: size + 'px' }"/>
		<div>
			<div class="lockup__overline">{{ overline }}</div>
			<div class="lockup__name" :style="{ fontSize: Math.round(size * 0.62) + 'px' }">KIT<span>WOLF</span></div>
		</div>
	</div>
</template>

<style scoped>
.lockup {
	display: flex;
	align-items: center;
	gap: 12px;
}
.lockup__overline {
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	font-size: var(--wd-size-overline);
	letter-spacing: var(--wd-track-overline);
	text-transform: uppercase;
	color: var(--text-accent);
	line-height: 1.2;
}
.lockup__name {
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	letter-spacing: var(--wd-track-heading);
	line-height: var(--wd-leading-display);
	color: var(--text-display);
}
.lockup__name span {
	color: var(--wd-gold-500);
}
</style>

<script>
import mark from '../../assets/brand/kitdog-mark-white.svg';

export default {
	props: {
		size: { type: Number, default: 44 },
		overline: { type: String, default: 'Wardogs utility' }
	},
	data() {
		return { mark }
	}
}
</script>
