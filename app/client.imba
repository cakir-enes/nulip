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
	
	query = ""

	def filter
		emit 'optsChange', Array.from({length: 200}).fill("a")

	<self[d:flex]>
		<input[bg:none bd:none c:white p:2] 
			bind=query
			@input.debounce=filter
			@focus.emit-focused('stream')
			@blur.emit-blured('stream')
			placeholder="Switch Stream">



		

tag app

	clippyOn = false

	<self[d:hflex bg:$background] @keydown.enter.prevent=(clippyOn = !clippyOn)>
		<div[d:vflex bg:$background2  h:100vh w:80ch]>
			if clippyOn 
				<clippy>
			else 
				<timeline>
				
			<clippy>
				<StreamSwitcher>
			<editor>

imba.mount <app>