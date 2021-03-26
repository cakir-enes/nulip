tag timeline
	prop items = [{
			id: "asd", kind: "thread", title: "thread#1", stream: "learn", topic: "zig", postedAt: (new Date()).toLocaleString()
		}, {
			id: "asd2", content: "bloccck", stream: "learn", topic: "crystal", postedAt: (new Date()).toLocaleString()
			}]

	<self[d:vflex flg:1 of:auto c:white]
		@toggleSelect=(do $1.detail.selected = !($1.detail.selected))>
		<span> JSON.stringify items.filter(do $1.selected).map do $1.id
		for item in items
			<Block item=item>


tag Block < div
	prop item
	
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
		@mouseenter=(do showControls = true)
		@mouseleave=(do showControls = false)>
		console.log showControls
		<div @click.emit-toggleSelect(item)>
			if item.kind is "thread"
				thread!
			else
				block!
			info!
		<div[d:hflex pos:absolute t:0 r:0 h:100% zi:2 bg:black o:0 ]
			[o:0.7]=showControls>
			<button @click.emit-edit(item)> "Edit"
			<button @click.emit-delete(item)> "Del"


	
		