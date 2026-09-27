<!-- Port of template/components/core/Panel: opaque chamfered plate with an optional label bar. -->
<template>
	<section class="wd-panel">
		<header v-if="label || $slots.actions" class="wd-panel__header">
			<span class="wd-panel__label">{{ label }}</span>
			<div class="wd-panel__actions"><slot name="actions"/></div>
		</header>
		<div class="wd-panel__body" :style="{ padding: inset + 'px' }">
			<slot/>
		</div>
	</section>
</template>

<style scoped>
.wd-panel {
	background: var(--surface-panel);
	border: 1px solid var(--line-hair);
	clip-path: polygon(12px 0, 100% 0, 100% calc(100% - 12px), calc(100% - 12px) 100%, 0 100%, 0 12px);
	box-shadow: var(--wd-shadow-plate);
}
.wd-panel__header {
	display: flex;
	align-items: center;
	justify-content: space-between;
	gap: 12px;
	padding: 10px 16px;
	border-bottom: 1px solid var(--line-hair);
	background: var(--wd-black-a40);
}
.wd-panel__label {
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	font-size: var(--wd-size-overline);
	letter-spacing: var(--wd-track-plate);
	text-transform: uppercase;
	color: var(--text-muted);
}
.wd-panel__actions {
	display: flex;
	align-items: center;
	gap: 8px;
}
</style>

<script>
export default {
	props: {
		label: { type: String, default: '' },
		inset: { type: Number, default: 16 }
	}
}
</script>
