
(define (recurse n r)
	(if (> n 0)
		(recurse (- n 1) (+ r 5))
		r
	)	
)



(recurse 3 0)


