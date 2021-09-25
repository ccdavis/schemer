(define x 25)
(define y 30)
(define (test  a b c)
	(
		(define d 99)
		(define e 101)
		(if (> a 1) 
			(+ a b c d e)
			(+ e d)
		)
	)
)

(test 1 2 3)
(test 9 9 9)

(define (test-define-exps a b c)
	(
		(define sum_abc (+ a b c))
		(if (> a 1)
			sum_abc
			99
		)
	)
)

(test-define-exps 1 2 3)
(test-define-exps 5 2 3)


(define (fib n)
	(
		(if (> 2 n)
			(if (> n 0)
				1
				0
			)
			
			(+ (fib (- n 1)) (fib (- n 2)))
		)
	)
)

(fib 5)
(fib 8)
(fib 11)
(fib 20)
