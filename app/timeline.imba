tag timeline
	prop items = [{
			id: "asd", kind: "thread", title: "thread#1", stream: "learn", topic: "zig", postedAt: (new Date()).toLocaleString()
		}, {
			id: "asd2", content: "bloccck", stream: "learn", topic: "crystal", postedAt: (new Date()).toLocaleString()
			}]

	<self[d:vflex flg:1 of:auto]>
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
			<button> "Edit"
			<button> "Del"
			

	def thread
		<>
			<h2> item.title
			<span> item.content
	
	def block
		<>
			<span> item.content
	
	<self[d:vflex p:2 bg:$background c:$text m:2 mt:2px]>
		if item.kind is "thread"
			<.thread> 
				thread!
				info!
		else
			<.block> 
				block!
				info!



	
		