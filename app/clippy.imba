tag clippy
	css
		* transition:height 0.15s ease-out

	focused = 0
	opts = []
	show = false

	def focusDown
		focused += if focused < (opts.length - 1) then 1 else 0
		document.getElementById("elem-{focused}").scrollIntoView false
		
	
	def focusUp
		focused -= if focused > 0 then 1 else 0

	
	<self[d:vflex ff:mono bg:$background c:#686c6f fld:column-reverse rdtr:8 rdtl:8 max-height:60vh] [rdtr:0 rdtl:0]=!show
		@keydown.up.prevent.focusUp
		@keydown.down.prevent.focusDown>
		
		<div[d:hflex pos:sticky b:0 l:0] 
			@focused=(do show = true)
			@blured=(do show = false; opts = [])
			@optsChange.prevent=(do opts = $1.detail)>
			<SearchFilter>
			<StreamFilter>
		
		<ul[of:auto p:0 mt:0 h:0vh] tabindex=-1 tabIndex=-1 [p:2 mt:4 h:100%]=show> for opt, i in opts
			<li id="elem-{i}" [bg:black c:white]=(focused is i)> <span> "{i} - {opt}"



tag SearchFilter < div
	query = ""

	def filter
		emit 'optsChange', ['asd', 'bzxc']

	def onBlur
		query = ""
		emit 'blured', 'search'


	<self[d:flex bd:4px solid black p:2px]>
		<input[bg:none bd:3px solid black c:white p:2 ff:mono]
			bind=query
			@input.debounce=filter
			@focus.emit-focused('search')
			@blur=onBlur
			placeholder="Search..."
		>


		
tag StreamFilter < div
	
	query = ""

	def filter
		emit 'optsChange', Array.from({length: 200}).fill("a")

	def onBlur
		query = ""
		emit 'blured', 'stream'

	<self[d:flex]>
		<input[bg:none bd:none c:white p:2 ff:mono] 
			bind=query
			@input.debounce=filter
			@focus.emit-focused('stream')
			@blur=onBlur
			placeholder="Switch Stream">
		