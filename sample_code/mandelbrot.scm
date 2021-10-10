
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
	(output c-x c-iy)
		
	(while (and (< iterations max-iterations)
				(not-escaped? zn-x zn-iy))
				
		(set! tmp-zx (+ c-x 
					(- (* zn-x zn-x)
						(* zn-iy zn-iy))))
							
		(set! zn-iy (+ c-iy 
					(* 2.0 (* zn-x zn-iy))))
						
		(set! zn-x tmp-zx)
		(set! iterations (+ iterations 1))														
	)
	(< iterations max-iterations)			
)


(define (calc x1 y1 x2 y2)	
(
	(define left x1)
	(output 7 7 7 7)
	(define right x2)
	(define top y1)
	(define bottom y2)
		
	(output 8 8 8 8 8 8)
	
	(define x-increment 0.0)
	(define y-increment 0.0)
	(define height 0.0)
	(define width 0.0)
	(output 2 2 2 2 2)
	(set! width (- right left))
	(output 3 3 3 3)
	(set! height (- bottom top))
		
	(set! x-increment (/ width x-resolution))
	(set! y-increment (/ height y-resolution))
	
	(define points 0)
	(output 9 9 9 9)
	
	(while (< left right)				
		(while (< top bottom)
			(in-mandelbrot left top)			
			(set! top (+ top y-increment))
			(set! points (+ points 1))
		)
		(set! left (+ left x-increment))
		(output left)
		(output points)
	)	
)
)

(output 5 5 5 5)
(calc -1.5 -1.0 1.0 1.0)



