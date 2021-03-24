tag clippy

	focused = 0
	opts = []

	def focusDown
		focused += if focused < 2 then 1 else 0
		console.log "FOCUS DOWN {focused}"
	
	def focusUp
		focused -= if focused > 0 then 1 else 0


	<self[d:vflex fld:column-reverse flg:1] 
		@keydown.up.prevent.focusUp
		@keydown.down.prevent.focusDown>
		
		<div[d:hflex pos:relative] @optsChange.prevent=(do opts = $1.detail)>
			<input[pl:2ch bg:white w:100% c:black] type="text" placeholder="Search">
			<slot>
			<span[pos:absolute l:4px t:0px]> ">"
		<ul> for opt, i in opts
			<li [bg:black c:white]=(focused is i)> <a> opt

		
		