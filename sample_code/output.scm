
(define (test a b c)
	((output a)
	(if (< a 3)
		(output a b c)		
		(test (- a 1) (- b 1) (- c 1)))
	)
)

(test 95 5 5)

