<!-- Port of template/components/forms/Input. -->
<template>
	<label class="wd-input">
		<span v-if="label" class="wd-input__label">{{ label }}</span>
		<span class="wd-input__field" :class="{ 'wd-input__field--error': error }">
			<input ref="input" :value="modelValue" v-bind="$attrs" @input="$emit('update:modelValue', $event.target.value)"/>
		</span>
		<span v-if="error || hint" class="wd-input__hint" :class="{ 'wd-input__hint--error': error }">{{ error || hint }}</span>
	</label>
</template>

<style scoped>
.wd-input {
	display: flex;
	flex-direction: column;
	gap: 6px;
}
.wd-input__label {
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	font-size: var(--wd-size-overline);
	letter-spacing: var(--wd-track-overline);
	text-transform: uppercase;
	color: var(--text-muted);
}
.wd-input__field {
	display: flex;
	align-items: center;
	padding: 0 10px;
	background: var(--surface-input);
	border: 1px solid var(--line-hair);
	border-radius: var(--wd-radius-1);
	transition: var(--wd-transition-control);
}
.wd-input__field:focus-within {
	border-color: var(--wd-gold-500);
	box-shadow: inset 0 0 0 1px var(--wd-gold-a20);
}
.wd-input__field--error { border-color: var(--wd-alert-500); }
.wd-input__field input {
	all: unset;
	flex: 1;
	min-width: 0;
	padding: 9px 0;
	font-family: var(--wd-font-ui);
	font-size: var(--wd-size-body);
	color: var(--text-display);
	user-select: text;
}
.wd-input__hint {
	font-size: var(--wd-size-body-sm);
	color: var(--text-faint);
}
.wd-input__hint--error { color: #E88178; }
</style>

<script>
export default {
	inheritAttrs: false,
	props: {
		modelValue: { type: String, default: '' },
		label: { type: String, default: '' },
		hint: { type: String, default: '' },
		error: { type: String, default: '' }
	},
	emits: ['update:modelValue'],
	methods: {
		focus: function(){
			this.$refs.input.focus();
		}
	}
}
</script>
