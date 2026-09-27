<!-- Port of template/components/core/Badge. -->
<template>
	<span class="wd-badge" :class="`wd-badge--${tone}`">
		<span v-if="dot" class="wd-badge__dot"></span>
		<slot/>
	</span>
</template>

<style scoped>
.wd-badge {
	display: inline-flex;
	align-items: center;
	gap: 6px;
	padding: 3px 8px;
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	font-size: var(--wd-size-overline);
	letter-spacing: var(--wd-track-label);
	text-transform: uppercase;
	line-height: 1.4;
	border: 1px solid;
}
.wd-badge__dot { width: 6px; height: 6px; background: currentColor; }

.wd-badge--neutral { color: var(--text-body); border-color: var(--line-hair); background: var(--wd-steel-800); }
.wd-badge--objective { color: var(--wd-gold-500); border-color: var(--wd-gold-a45); background: var(--wd-gold-a08); }
.wd-badge--friendly { color: #B6C97A; border-color: rgba(122,139,58,.5); background: rgba(122,139,58,.14); }
.wd-badge--hostile { color: #E88178; border-color: rgba(195,55,44,.5); background: rgba(195,55,44,.14); }
</style>

<script>
export default {
	props: {
		tone: { type: String, default: 'neutral' },
		dot: { type: Boolean, default: false }
	}
}
</script>
