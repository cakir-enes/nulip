import "./timeline"
import "./editor"
import "./clippy"

global css html
	ff:sans
	$background: #06090f
	$background2:#0d1117
	$text: #c9d1d9
	$text2:#8b949e
	$leaf:#7ee787


tag StreamSwitcher < div
	
	<self[d:flex]>
		<input[bg:none bd:none c:white p:2] @focus.log("ahhhha") placeholder="Switch Stream">



		

tag app

	clippyOn = false

	<self[d:hflex bg:$background] @keydown.enter.prevent=(clippyOn = !clippyOn)>
		<div[d:vflex bg:$background2  h:100vh w:80ch]>
			if clippyOn 
				<clippy>
			else 
				<timeline>
				
			<clippy>
				<button @click.emit('optsChange', ["kekekek", "2"])>
				<StreamSwitcher>
			<editor>

imba.mount <app>