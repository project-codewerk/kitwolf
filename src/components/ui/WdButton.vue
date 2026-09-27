<!-- Port of template/components/core/Button. -->
<template>
	<button class="wd-button" :class="[`wd-button--${variant}`, `wd-button--${size}`, { 'wd-button--full': fullWidth }]" :disabled="disabled">
		<WdIcon v-if="icon" :name="icon" :size="size == 'sm' ? 14 : 16"/>
		<span v-if="slashes" class="wd-button__slashes">//</span>
		<slot/>
	</button>
</template>

<style scoped>
.wd-button {
	--chamfer: 8px;
	display: inline-flex;
	align-items: center;
	justify-content: center;
	gap: 10px;
	padding: 11px 22px;
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	font-size: 14px;
	letter-spacing: var(--wd-track-label);
	text-transform: uppercase;
	line-height: 1;
	cursor: pointer;
	clip-path: polygon(var(--chamfer) 0, 100% 0, 100% calc(100% - var(--chamfer)), calc(100% - var(--chamfer)) 100%, 0 100%, 0 var(--chamfer));
	box-shadow: var(--wd-shadow-plate);
	transition: var(--wd-transition-control);
}
.wd-button--sm { --chamfer: 6px; padding: 7px 14px; font-size: 12px; }
.wd-button--lg { --chamfer: 10px; padding: 15px 32px; font-size: 17px; }
.wd-button--full { width: 100%; }
.wd-button:active:not(:disabled) { transform: translateY(1px); box-shadow: none; }
.wd-button:focus-visible { outline: none; box-shadow: var(--wd-focus-ring); }

.wd-button--primary { background: var(--wd-gold-fill); color: var(--wd-black); border: 1px solid var(--wd-gold-600); }
.wd-button--primary:hover:not(:disabled) { background: var(--wd-gold-300); border-color: var(--wd-gold-500); }
.wd-button--primary .wd-button__slashes { opacity: .55; }

.wd-button--secondary { background: var(--wd-plate-fill); color: var(--wd-bone-050); border: 1px solid var(--wd-gold-a45); }
.wd-button--secondary:hover:not(:disabled) { background: var(--wd-brass-700); border-color: var(--wd-gold-500); }

.wd-button--ghost { background: transparent; color: var(--wd-bone-100); border: 1px solid var(--wd-steel-500); }
.wd-button--ghost:hover:not(:disabled) { background: var(--wd-white-a06); border-color: var(--wd-gold-500); color: var(--wd-gold-300); }

.wd-button--danger { background: linear-gradient(180deg, var(--wd-alert-500) 0%, var(--wd-alert-600) 100%); color: var(--wd-bone-050); border: 1px solid var(--wd-alert-600); }
.wd-button--danger:hover:not(:disabled) { background: var(--wd-alert-500); border-color: var(--wd-alert-500); }

.wd-button__slashes { color: var(--wd-gold-500); }

.wd-button:disabled {
	background: var(--surface-disabled);
	color: var(--text-disabled);
	border: 1px solid var(--wd-steel-600);
	box-shadow: none;
	cursor: not-allowed;
}
</style>

<script>
import WdIcon from './WdIcon.vue';

export default {
	components: { WdIcon },
	props: {
		variant: { type: String, default: 'primary' },
		size: { type: String, default: 'md' },
		icon: { type: String, default: null },
		slashes: { type: Boolean, default: false },
		fullWidth: { type: Boolean, default: false },
		disabled: { type: Boolean, default: false }
	}
}
</script>
