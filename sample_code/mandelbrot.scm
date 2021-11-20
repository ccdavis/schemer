
(define x-resolution 200)
(define y-resolution 320)
(define max-iterations 25)
(define escape-radius 2.0)
	
(define (not-escaped? x iy)
	(begin
	(< (+ (* x x) (* iy iy)) (* 2 escape-radius))
	)
)

(define (in-mandelbrot c-x c-iy)
(begin
	(define iterations 0)
	(define zn-x 0.0)
	(define zn-iy 0.0)
	
	(define tmp-zx 0.0)	
		
	(while (and (< iterations max-iterations)
				(not-escaped? zn-x zn-iy))
	(begin
				
		(set! tmp-zx (+ c-x 
					(- (* zn-x zn-x)
						(* zn-iy zn-iy))))
							
		(set! zn-iy (+ c-iy 
					(* 2.0 (* zn-x zn-iy))))
						
		(set! zn-x tmp-zx)
		(set! iterations (+ iterations 1))																
	))
	(< iterations max-iterations)			
))

(define (calc x1 y1 x2 y2)	
(begin
	(define left x1)	
	(define right x2)
	(define top y1)
	(define bottom y2)
				
	(define height (- bottom top))
	(define width (- right left))
	(define x-increment (/ width x-resolution))
	(define y-increment (/ height y-resolution))
		
	(define points 0)
	(define columns 0)
	
	(output points)
	
	(while (< left right)				
	(begin
		(set! columns (+ columns 1))
		(while (< top bottom)
		(begin
			(in-mandelbrot left top)			
			(set! top (+ top y-increment))
			(set! points (+ points 1))						
		))
		
		(set! top y1)		
		(set! left (+ left x-increment))				
	))
))

(calc -1.5 -1.0 1.0 1.0)


