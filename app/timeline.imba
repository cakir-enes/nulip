tag timeline
	prop items = [{
			id: "asd", kind: "thread", title: "thread#1", stream: "learn", topic: "zig", postedAt: (new Date()).toLocaleString()
		}, {
			id: "asd2", content: "bloccck", stream: "learn", topic: "crystal", postedAt: (new Date()).toLocaleString()
			}]
	
	focused = null 


	def focusDown
		console.log focused
		focused = focused < (items.length - 1) ? focused + 1 : 0 

	def focusUp
		console.log focused
		focused = focused > 0 ? focused - 1 : 0

	def focusLast
		focused = items.length - 1

	def toggleFocused
		emit('toggleSelect', items[focused] ?? {})

	<self[d:vflex flg:1 of:auto c:white bd@focus:2px solid blue4]
		tabindex=0
		@focus=focusLast
		@blur=(do focused = null)
		@blockClick=(do focused = $1.detail; toggleFocused!)
		@keydown.down.prevent=focusDown
		@keydown.up.prevent=focusUp
		@keydown.space=toggleFocused>

		<span> JSON.stringify items.filter(do $1.selected).map do $1.id
		<ul>
			for item, i in items
				<Block i=i item=item focused=(focused is i)>


tag Block < li
	prop item
	prop focused
	prop i
	
	def info
		<div[d:hflex p:2px bg:$background]>
			css span mr:2
			<span> item.stream
			<span> item.topic
			<span[ml:auto]> item.postedAt
			

	def thread
		<>
			<h2> item.title
			<span> item.content
	
	def block
		<>
			<span> item.content

	showControls = false
	
	<self[d:vflex pos:relative p:2 bg:$background c:$text m:2 mt:2px]
		[bg:blue4]=item.selected
		[bg:blue8]=focused
		@mouseenter=(do showControls = true)
		@mouseleave=(do showControls = false)>
		
		<div @click.emit-blockClick(i)>
			if item.kind is "thread"
				thread!
			else
				block!
			info!
		<div[d:hflex pos:absolute t:0 r:0 h:100% zi:2 bg:black o:0 ]
			[o:0.7]=showControls>
			<span @click.emit-edit(item)> "Edit"
			<span @click.emit-delete(item)> "Del"


	
		