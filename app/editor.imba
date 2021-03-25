tag editor

	<self[d:vflex]>
		css button bg:$background bd:none c:$leaf mr:2 py:1 px:2 bxs:sm
		<textarea[w:100% fs:sm h:20ch bg:$background2 c:#ff7b72 ff: 'Courier Prime', monospace]>


tag WithChar < div
	prop char
	prop el
	
	<self[d:hflex pos:relative ff:mono]>
		<span[pos:absolute t:-6px fs:lg l:0 c:orange3]> char
		<slot>	


