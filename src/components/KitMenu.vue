<!-- The F8 overlay: pick a kit to rebuy, record a new one, or name a finished recording. -->
<template>
	<div class="scrim" @mousedown.self="commands.closeMenu()">
		<div class="menu">
			<header class="menu__header wd-camo">
				<KitwolfLockup overline="Kit macros"/>
				<WdIconButton icon="x" label="Close (F8)" @click="commands.closeMenu()"/>
			</header>

			<!-- A finished recording: name it -->
			<section v-if="state.pending !== null" class="menu__body">
				<template v-if="state.pending > 0">
					<div>
						<div class="overline">Recording complete</div>
						<h2>Name this kit</h2>
						<div class="wd-mono menu__meta">{{ state.pending }} clicks captured<template v-if="state.pendingCtrl"> · {{ state.pendingCtrl }} ctrl</template></div>
					</div>
					<p v-if="!state.pendingCtrl" class="tip">
						<span class="tip__title">Replays at safe speed</span>
						No Ctrl+clicks were recorded, so Kitwolf can't tell which click opens the bag popup. Every click will wait 400 ms first.
						For a faster rebuy, discard and record again, with a <span class="wd-kbd">Ctrl</span>+click on the bag purchase.
					</p>
					<form class="naming" @submit.prevent="save">
						<WdInput ref="nameInput" v-model="name" label="Kit name" placeholder="e.g. Marksman + Ghillie"
							maxlength="40" :error="error" autocomplete="off" spellcheck="false"/>
						<div class="actions">
							<WdButton type="button" variant="ghost" size="sm" @click="commands.discardRecording()">Discard</WdButton>
							<WdButton type="submit" size="sm" icon="check" :disabled="!name.trim()">Save kit</WdButton>
						</div>
					</form>
				</template>
				<template v-else>
					<div class="alert">
						<WdIcon name="triangle-alert" :size="18"/>
						<div>
							<div class="alert__title">No clicks captured</div>
							Nothing was clicked in WARDOGS while recording.
						</div>
					</div>
					<div class="actions">
						<WdButton variant="ghost" size="sm" @click="commands.discardRecording()">Discard</WdButton>
						<WdButton variant="secondary" size="sm" icon="circle-dot" @click="commands.startRecording()">Record again</WdButton>
					</div>
				</template>
			</section>

			<!-- Saved kits -->
			<section v-else class="menu__body">
				<WdPanel label="Saved kits" :inset="0">
					<template #actions>
						<span class="wd-mono menu__meta">{{ String(state.kits.length).padStart(2, '0') }}</span>
					</template>
					<div v-if="state.kits.length" class="kits">
						<button v-for="(kit, ki) in state.kits" :key="kit.id" class="kit" @click="commands.replayKit(kit.id)">
							<span class="wd-kbd">{{ ki < 9 ? ki + 1 : '' }}</span>
							<span class="kit__name">{{ kit.name }}</span>
							<span class="kit__clicks wd-mono">{{ describeClicks(kit) }}</span>
							<span class="kit__rebuy"><WdIcon name="play" :size="12"/>Rebuy</span>
						</button>
					</div>
					<div v-else class="empty">
						<div class="empty__title">No kits banked</div>
						Record your buy once. Next life it's one keypress away.
					</div>
				</WdPanel>

				<WdButton variant="secondary" icon="circle-dot" full-width @click="commands.startRecording()">Record new kit</WdButton>
				<p class="hint">Click through the buy menu as usual. Every left click is recorded. Press <span class="wd-kbd">F8</span> when done.</p>
				<p class="tip">
					<span class="tip__title"><span class="wd-kbd">Ctrl</span>+click the bag</span>
					Buying a bag opens a popup, and the next click has to wait for it. Hold <span class="wd-kbd">Ctrl</span> when you click to buy the bag.
					On replay only that click waits an extra 400 ms, and the rest go through fast.
					Kitwolf can't tell what each click does, so a kit without a Ctrl+click waits before every click.
				</p>
			</section>

			<footer class="menu__footer">
				<span><span class="wd-kbd">F8</span> / <span class="wd-kbd">Esc</span> Close</span>
				<span v-if="state.pending === null && state.kits.length"><span class="wd-kbd">1</span>–<span class="wd-kbd">9</span> Rebuy</span>
			</footer>
		</div>
	</div>
</template>

<style scoped>
.scrim {
	position: absolute;
	inset: 0;
	display: flex;
	align-items: center;
	justify-content: center;
	background: rgba(7, 8, 9, .78);
	animation: fade-in var(--wd-dur-base) var(--wd-ease-mech);
}
.menu {
	width: 520px;
	max-width: 92%;
	background: var(--surface-card);
	border: 1px solid var(--line-strong);
	box-shadow: var(--wd-shadow-drop-modal);
	clip-path: polygon(16px 0, 100% 0, 100% calc(100% - 16px), calc(100% - 16px) 100%, 0 100%, 0 16px);
	animation: seat-in var(--wd-dur-base) var(--wd-ease-mech);
}
.menu__header {
	display: flex;
	align-items: center;
	justify-content: space-between;
	padding: 16px 20px;
	border-bottom: 1px solid var(--line-hair);
}
.menu__body {
	display: flex;
	flex-direction: column;
	gap: 16px;
	padding: 20px;
}
.menu__meta {
	text-transform: uppercase;
	color: var(--wd-gold-500);
}
.menu__footer {
	display: flex;
	justify-content: space-between;
	padding: 10px 20px;
	border-top: 1px solid var(--line-hair);
	background: var(--wd-black-a40);
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	font-size: var(--wd-size-overline);
	letter-spacing: var(--wd-track-label);
	text-transform: uppercase;
	color: var(--text-muted);
}
.overline {
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	font-size: var(--wd-size-overline);
	letter-spacing: var(--wd-track-overline);
	text-transform: uppercase;
	color: var(--text-accent);
	margin-bottom: 4px;
}
h2 {
	font-size: var(--wd-size-heading-2);
}

.kits {
	display: flex;
	flex-direction: column;
	max-height: 50vh;
	overflow-y: auto;
}
.kit {
	all: unset;
	display: flex;
	align-items: center;
	gap: 12px;
	height: 44px;
	padding: 0 16px;
	border-bottom: 1px solid var(--line-hair);
	cursor: pointer;
	transition: var(--wd-transition-control);
}
.kit:last-child { border-bottom: none; }
.kit:hover, .kit:focus-visible { background: var(--surface-hover); }
.kit:hover .kit__rebuy, .kit:focus-visible .kit__rebuy { color: var(--wd-gold-300); }
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
.kit__rebuy {
	display: inline-flex;
	align-items: center;
	gap: 6px;
	width: 76px;
	justify-content: flex-end;
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	font-size: var(--wd-size-label);
	letter-spacing: var(--wd-track-label);
	text-transform: uppercase;
	color: var(--wd-gold-500);
}

.empty {
	padding: 20px 16px;
	font-size: var(--wd-size-body-sm);
}
.empty__title, .alert__title {
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	letter-spacing: var(--wd-track-label);
	text-transform: uppercase;
	color: var(--text-display);
}
.tip {
	margin: 0;
	padding: 10px 14px;
	font-size: var(--wd-size-body-sm);
	color: var(--text-body);
	background: var(--wd-gold-a08);
	border-left: var(--wd-border-heavy) solid var(--wd-gold-500);
}
.tip__title {
	display: block;
	margin-bottom: 2px;
	font-family: var(--wd-font-display);
	font-weight: var(--wd-weight-bold);
	letter-spacing: var(--wd-track-label);
	text-transform: uppercase;
	color: var(--text-display);
}
.hint {
	margin: -4px 0 0;
	font-size: var(--wd-size-body-sm);
	color: var(--text-muted);
}

.naming {
	display: flex;
	flex-direction: column;
	gap: 16px;
}
.actions {
	display: flex;
	justify-content: flex-end;
	gap: 10px;
}
.alert {
	display: flex;
	gap: 12px;
	padding: 12px 14px;
	color: var(--text-body);
	background: rgba(195, 55, 44, .1);
	border-left: var(--wd-border-heavy) solid var(--wd-alert-500);
}
.alert > .wd-icon { color: #E88178; margin-top: 2px; }

@keyframes fade-in {
	from { opacity: 0; }
}
@keyframes seat-in {
	from { opacity: 0; transform: translateY(6px); }
}
</style>

<script>
import WdButton from './ui/WdButton.vue';
import WdIcon from './ui/WdIcon.vue';
import WdIconButton from './ui/WdIconButton.vue';
import WdInput from './ui/WdInput.vue';
import WdPanel from './ui/WdPanel.vue';
import KitwolfLockup from './ui/KitwolfLockup.vue';
import { state, commands, describeClicks } from '../helpers/kitwolf.js';

export default {
	components: { WdButton, WdIcon, WdIconButton, WdInput, WdPanel, KitwolfLockup },
	data() {
		return {
			state,
			commands,
			name: '',
			error: '',
			f8Pressed: false
		}
	},
	mounted: function(){
		window.addEventListener('keydown', this.onKeydown);
		window.addEventListener('keyup', this.onKeyup);
		this.focusName();
	},
	unmounted: function(){
		window.removeEventListener('keydown', this.onKeydown);
		window.removeEventListener('keyup', this.onKeyup);
	},
	watch: {
		'state.pending': function(){
			this.name = '';
			this.error = '';
			this.focusName();
		}
	},
	methods: {
		describeClicks,
		focusName: function(){
			this.$nextTick(() => {
				if(this.$refs.nameInput){
					this.$refs.nameInput.focus();
				}
			});
		},
		onKeydown: function(event){
			// The global F8 hook can't see keys while the menu has focus, so F8 is handled here.
			if(event.key == 'F8'){
				event.preventDefault();
				if(!event.repeat){
					this.f8Pressed = true;
				}
				return;
			}
			if(event.key == 'Escape'){
				commands.closeMenu();
				return;
			}
			// Number keys rebuy straight from the list (not while typing a name).
			if(state.pending === null && /^[1-9]$/.test(event.key)){
				const kit = state.kits[Number(event.key) - 1];
				if(kit){
					commands.replayKit(kit.id);
				}
			}
		},
		onKeyup: function(event){
			// Close on release, and only for a press that started here: the release of the F8
			// that opened the menu also lands here, and the key is fully up by the time the
			// game gets focus back (no auto-repeat reopening it).
			if(event.key == 'F8' && this.f8Pressed){
				this.f8Pressed = false;
				commands.closeMenu();
			}
		},
		save: async function(){
			try {
				await commands.saveRecording(this.name);
			} catch(e) {
				this.error = String(e);
			}
		}
	}
}
</script>
