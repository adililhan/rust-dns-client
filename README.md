#  DNS Client For Fun

MX record:

    root@main: ./rust-dns google.com mx
	google.com.		38S IN MX	10 smtp.google.com.

TXT record:

	root@main: ./rust-dns google.com txt
	google.com.		1H IN TXT	"globalsign-smime-dv=CDYX+XFHUw2wml6/Gb8+59BsH31KzUr6c1l2BPvqKX8="
	google.com.		1H IN TXT	"onetrust-domain-verification=de01ed21f2fa4d8781cbc3ffb89cf4ef"
	google.com.		1H IN TXT	"v=spf1 include:_spf.google.com ~all"
	google.com.		1H IN TXT	"facebook-domain-verification=22rm551cu4k0ab0bxsw536tlds4h95"
	google.com.		1H IN TXT	"google-site-verification=TV9-DBe4R80X4v0M4U_bd_J9cpOJM0nikft0jAgjmsQ"
	google.com.		1H IN TXT	"MS=E4A68B9AB2BB9670BCE15412F62916164C0B20BB"
	google.com.		1H IN TXT	"atlassian-domain-verification=5YjTmWmjI92ewqkx2oXmBaD60Td9zWon9r6eakvHX6B77zzkFQto8PQ9QsKnbf4I"
	google.com.		1H IN TXT	"google-site-verification=wD8N7i1JTNTkezJ49swvWW48f8_9xveREV4oB-0Hf5o"
	google.com.		1H IN TXT	"docusign=1b0a6754-49b1-4db5-8540-d2c12664b289"
	google.com.		1H IN TXT	"webexdomainverification.8YX6G=6e6922db-e3e6-4a36-904e-a805c28087fa"
	google.com.		1H IN TXT	"apple-domain-verification=30afIBcvSuDV2PLX"
	google.com.		1H IN TXT	"docusign=05958488-4752-4ef2-95eb-aa7ba8a3bd0e"

IPv6:
	
	root@main: ./rust-dns yahoo.com aaaa
	yahoo.com.		1m30s IN AAAA	2001:4998:44:3507::8001
	yahoo.com.		1m30s IN AAAA	2001:4998:124:1507::f000
	yahoo.com.		1m30s IN AAAA	2001:4998:24:120d::1:0
	yahoo.com.		1m30s IN AAAA	2001:4998:44:3507::8000
	yahoo.com.		1m30s IN AAAA	2001:4998:124:1507::f001
	yahoo.com.		1m30s IN AAAA	2001:4998:24:120d::1:1

