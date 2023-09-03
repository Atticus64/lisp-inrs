
(define age 50) 
(define old "Youre Old")
(define young "Youre Young")
(define res (lambda (age) (if (>= age 40) old young)))
(res 20)
(print age)

