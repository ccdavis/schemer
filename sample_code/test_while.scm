
(define counter 10)

(define (test-while x) 
(begin
	(output 1)
	(while (> x 0) 
		(begin
			(output x) 
			(set! x (- x 1))
		)
	)
)
)

(test-while 5)

