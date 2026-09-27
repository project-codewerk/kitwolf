<!-- Port of template/components/core/Icon: Lucide SVG used as a mask so it takes currentColor. -->
<template>
	<span class="wd-icon" aria-hidden="true" :style="{
		width: size + 'px',
		height: size + 'px',
		maskImage: `url(${icons[name]})`,
		WebkitMaskImage: `url(${icons[name]})`
	}"></span>
</template>

<style scoped>
.wd-icon {
	display: inline-block;
	flex: 0 0 auto;
	background-color: currentColor;
	mask-repeat: no-repeat;
	-webkit-mask-repeat: no-repeat;
	mask-size: contain;
	-webkit-mask-size: contain;
	mask-position: center;
	-webkit-mask-position: center;
}
</style>

<script>
import backpack from '../../assets/icons/backpack.svg';
import check from '../../assets/icons/check.svg';
import circleDot from '../../assets/icons/circle-dot.svg';
import crosshair from '../../assets/icons/crosshair.svg';
import play from '../../assets/icons/play.svg';
import triangleAlert from '../../assets/icons/triangle-alert.svg';
import x from '../../assets/icons/x.svg';

const icons = { backpack, check, 'circle-dot': circleDot, crosshair, play, 'triangle-alert': triangleAlert, x };

export default {
	props: {
		name: { type: String, required: true },
		size: { type: Number, default: 16 }
	},
	data() {
		return { icons }
	}
}
</script>
