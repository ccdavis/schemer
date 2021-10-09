
(define x-resolution 480)
(define y-resolution 640)
(define max-iterations 255)
(define escape-radius 2.0)
	
(define (not-escaped? x iy)
	(< (+ (* x x) (* iy iy)) (* 2 escape-radius))
)

(define (in-mandelbrot c-x c-iy)
	(define iterations 0)
	(define zn-x 0.0)
	(define zn-iy 0.0)
	
	(define tmp-zx 0.0)
		
	(while (and (< iterations max-iterations)
				(not-escaped? zn-x zn-iy))
				
		(set! tmp-zx (+ c-x 
						( - (* zn-x zn-x)
							(* zn-iy zn-iy)))
		(set! zn-iy (+ c-iy 
						(* 2.0 (* zn-x zn-iy)))
		(set! zn-x tmp-zx)
		(set! iterations (+ iterations 1))														
	)
	(< iterations max-iterations)		
)


(define (calc x1 y1 x2 y2)
	(define left x1)
	(define right x2)
	(define top y1)
	(define bottom y2)
	
	(define x-increment 0.0)
	(define y-increment 0.0)
	
	(set! x-increment (/ (- right left) x-resolution))
	(set! y-increment (/ (- top bottom) y-resolution))
	
	(while (< left right)				
		(while (< top bottom)
			(in-mandelbrot left top)			
			(set! top (+ top y-increment))
		)
		(set! left (+ left x-increment))
	)
)

(calc -1.5 -1.0 1.0 1.0)
