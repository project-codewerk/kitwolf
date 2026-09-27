<template>
	<div id="control">
		<header class="control__header wd-camo">
			<KitwolfLockup :size="52"/>
			<WdBadge :tone="modeBadge.tone" dot>{{ modeBadge.label }}</WdBadge>
		</header>

		<main class="control__body">
			<WdPanel label="How it works">
				<ol class="steps">
					<li>Run WARDOGS in <strong>borderless windowed</strong> mode.</li>
					<li>In game, press <span class="wd-kbd">F8</span> to open the overlay.</li>
					<li><strong>Record new kit</strong>, click through the buy menu, then <span class="wd-kbd">F8</span> to finish and name it.</li>
					<li><span class="wd-kbd">Ctrl</span>+click to <strong>buy the bag</strong>. Its popup needs a moment, so that click waits a bit longer on replay and the rest go through fast. Without a Ctrl+click, every click waits, to be safe.</li>
					<li>Next life: <span class="wd-kbd">F8</span>, then pick the kit (or press <span class="wd-kbd">1</span>–<span class="wd-kbd">9</span>) to rebuy.</li>
				</ol>
			</WdPanel>

			<WdPanel label="Saved kits" :inset="0">
				<template #actions>
					<span class="wd-mono count">{{ String(state.kits.length).padStart(2, '0') }}</span>
				</template>
				<div v-if="state.kits.length" class="kits">
					<div v-for="kit in state.kits" :key="kit.id" class="kit">
						<span class="kit__name">{{ kit.name }}</span>
						<span class="kit__clicks wd-mono">{{ describeClicks(kit) }}</span>
						<WdIconButton icon="x" size="sm" :label="'Delete ' + kit.name" @click="commands.deleteKit(kit.id)"/>
					</div>
				</div>
				<div v-else class="empty">No kits banked yet.</div>
			</WdPanel>
		</main>

		<footer class="control__footer">Fan-made. Not affiliated with Bulkhead. Closing this window quits Kitwolf.</footer>
	</div>
</template>

<style scoped>
#control {
	display: flex;
	flex-direction: column;
	height: 100vh;
	background: var(--surface-page);
}
.control__header {
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding: 20px 24px;
	border-bottom: 1px solid var(--line-hair);
}
.control__body {
	flex: 1;
	min-height: 0;
	display: flex;
	flex-direction: column;
	gap: 16px;
	padding: 20px 24px;
	overflow-y: auto;
}
.control__footer {
	padding: 10px 24px;
	border-top: 1px solid var(--line-hair);
	font-size: var(--wd-size-body-sm);
	color: var(--text-faint);
}

.steps {
	margin: 0;
	padding-left: 20px;
	display: flex;
	flex-direction: column;
	gap: 6px;
	font-size: var(--wd-size-body-sm);
}
.steps li::marker {
	font-family: var(--wd-font-mono);
	color: var(--wd-gold-500);
}
.steps strong {
	color: var(--text-display);
	font-weight: var(--wd-weight-semibold);
}

.count {
	color: var(--wd-gold-500);
}
.kit {
	display: flex;
	align-items: center;
	gap: 12px;
	height: 44px;
	padding: 0 8px 0 16px;
	border-bottom: 1px solid var(--line-hair);
}
.kit:last-child { border-bottom: none; }
.kit__name {
	flex: 1;
	min-width: 0;
	overflow: hidden;
	text-overflow: ellipsis;
	white-space: nowrap;
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	font-size: 15px;
	letter-spacing: var(--wd-track-heading);
	text-transform: uppercase;
	color: var(--text-display);
}
.kit__clicks {
	color: var(--text-muted);
	text-transform: uppercase;
}
.empty {
	padding: 16px;
	font-size: var(--wd-size-body-sm);
	color: var(--text-muted);
}
</style>

<script>
import WdBadge from './ui/WdBadge.vue';
import WdIconButton from './ui/WdIconButton.vue';
import WdPanel from './ui/WdPanel.vue';
import KitwolfLockup from './ui/KitwolfLockup.vue';
import { state, connect, commands, describeClicks } from '../helpers/kitwolf.js';

const MODE_BADGES = {
	idle: { label: 'Standing by', tone: 'neutral' },
	menu: { label: 'Overlay open', tone: 'objective' },
	recording: { label: 'Recording', tone: 'hostile' },
	replaying: { label: 'Rebuying', tone: 'friendly' },
};

export default {
	components: { WdBadge, WdIconButton, WdPanel, KitwolfLockup },
	data() {
		return { state, commands }
	},
	methods: {
		describeClicks
	},
	computed: {
		modeBadge: function(){
			return MODE_BADGES[this.state.mode] || MODE_BADGES.idle;
		}
	},
	mounted: function(){
		connect();
	}
}
</script>
