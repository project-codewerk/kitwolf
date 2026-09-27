<!-- Port of template/components/core/IconButton. -->
<template>
	<button class="wd-icon-button" :class="`wd-icon-button--${size}`" :aria-label="label" :title="label">
		<WdIcon :name="icon" :size="size == 'sm' ? 14 : 18"/>
	</button>
</template>

<style scoped>
.wd-icon-button {
	display: inline-flex;
	align-items: center;
	justify-content: center;
	width: 36px;
	height: 36px;
	padding: 0;
	background: transparent;
	color: var(--text-body);
	border: 1px solid var(--line-hair);
	clip-path: polygon(6px 0, 100% 0, 100% calc(100% - 6px), calc(100% - 6px) 100%, 0 100%, 0 6px);
	cursor: pointer;
	transition: var(--wd-transition-control);
}
.wd-icon-button--sm { width: 28px; height: 28px; }
.wd-icon-button:hover { background: var(--wd-white-a06); color: var(--wd-gold-300); border-color: var(--wd-gold-a45); }
.wd-icon-button:focus-visible { outline: none; box-shadow: var(--wd-focus-ring); }
</style>

<script>
import WdIcon from './WdIcon.vue';

export default {
	components: { WdIcon },
	props: {
		icon: { type: String, required: true },
		label: { type: String, required: true },
		size: { type: String, default: 'md' }
	}
}
</script>
