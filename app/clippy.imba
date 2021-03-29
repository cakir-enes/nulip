tag clippy
	css
		* transition:height 0.15s ease-out

	focused = 0
	opts = []
	show = null
	mode = ""

	def focusDown
		focused += if focused < (opts.length - 1) then 1 else 0
		document.getElementById("elem-{focused}").scrollIntoView false
		
	
	def focusUp
		focused -= if focused > 0 then 1 else 0


	def optsChange {detail: { opts, mode }}
		this.opts = opts
		this.mode = mode

	
	<self[d:vflex ff:mono bg:$background c:#686c6f fld:column-reverse rdtr:8 rdtl:8 max-height:60vh] 
		[rdtr:0 rdtl:0]=!show
		@keydown.up.prevent.focusUp
		@keydown.down.prevent.focusDown>
		
		<div[d:hflex pos:sticky b:0 l:0 jc:center] 
			@focused=(do show = $1.detail)
			@blured=(do show = null; opts = [])
			@optsChange.prevent=(do opts = $1.detail)>
			<MoveFilter>
			<SearchFilter>
			<StreamFilter>

			# <MoveFilter>
			# <MoveFilter>
			# 	<StreamFilter>
			# <StreamFilter>
		
		<ul[of:auto p:0 mt:0 h:0vh] tabindex=-1 tabIndex=-1 [p:2 mt:4 h:100%]=show> for opt, i in opts
			<li id="elem-{i}" [bg:black c:white]=(focused is i)> <span> "{i} - {opt}"


tag Filter < div
	css * tween:sizes expo-in-out 1s
	
	def focus
		this.style.width = 100%
	
	def blur
		this.style.width = "auto"
	
	<self[d:hflex c:white of:hidden max-width:250px bdr:4px solid black] @focused=focus @blured=blur tabindex=-1>
		<slot>



# tag SearchFilter < div
# 	query = ""

# 	def filter
# 		emit 'optsChange', ['asd', 'bzxc']

# 	def onBlur
# 		query = ""
# 		emit 'blured', 'search'

# 	def mount
# 		console.log "VII", $ffuuu
# 		super.ff = $ff

# 	<self[d:flex bd:4px solid black p:2px]>
# 		<input$ff[bg:none bd:3px solid black c:white p:2 ff:mono]
# 			bind=query
# 			@input.debounce=filter
# 			@focus.emit-focused('search')
# 			@blur=onBlur
# 			placeholder="Search..."
# 		>

# tag MoveFilter < div
# 	css * tween:all 1s

# 	show = false
	
# 	def focus
# 		show = true
# 		# $ff.focus!
# 		this.style.height = this.scrollHeight + "px"
# 		this.style.width = 100%
	
# 	def blur
# 		show = false
# 		this.style.height = 20px
# 		this.style.width = 70px
	
# 	<self[ta:center my:auto c:white h:20px of:hidden bg:red7 d:hflex fld:column-reverse w:70px] tabindex=0 @focus=focus @blured=blur>
# 		<span> "[M]ove"
# 		<span[w:120px h:100% ta:center bg:blue4] tabindex=0 @blur.emit-blured("vii") @focus.emit-focused("vii") tabindex=(show ? 0 : -1)> "XX"

# 		# <StreamFilter$ff [o:0]=!show tabindex=(show ? 0 : -1)>


tag SearchFilter < input
	css * tween:all 1s
	
	query = ""

	def filter
		emit 'optsChange', Array.from({length: 200}).fill("a")

	def onFocus
		this.style.width = "100%"
		emit 'focused', 'stream'

	def onBlur
		query = ""
		emit 'blured', 'stream'
		this.style.width = 10ch

	<self[d:flex bg:blue6 bd:none c:white p:2 ff:mono w:10ch]
		bind=query
		@input.debounce=filter
		@focus=onFocus
		@blur=onBlur
		placeholder="[S]earch">	

tag MoveFilter < input
	css * tween:all 1s
	
	query = ""

	def filter
		emit 'optsChange', Array.from({length: 200}).fill("a")

	def onFocus
		this.style.width = "100%"
		emit 'focused', 'stream'

	def onBlur
		query = ""
		emit 'blured', 'stream'
		this.style.width = 70px

	<self[d:flex bg:blue6 bd:none c:white p:2 ff:mono w:70px]
		bind=query
		@input.debounce=filter
		@focus=onFocus
		@blur=onBlur
		placeholder="[M]ove">	


tag StreamFilter < input
	css * tween:all 1s
	
	query = ""

	def filter
		emit 'optsChange', Array.from({length: 200}).fill("a")

	def onFocus
		emit 'focused', 'stream'

	def onBlur
		query = ""
		emit 'blured', 'stream'

	<self[d:flex bg:blue6 bd:none c:white p:2 ff:mono w:auto]
		bind=query
		@input.debounce=filter
		@focus=onFocus
		@blur=onBlur
		placeholder="Switch Streamaaaaaaaaa">	