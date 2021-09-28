
(define (fib n)	
	(if (> 2 n)
		(if (> n 0)
			1
			0
		)		
		(+ (fib (- n 1)) (fib (- n 2)))
	)
)

(fib 28)
