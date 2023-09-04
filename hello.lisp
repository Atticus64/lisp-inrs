(
 (define age 50) 
 (define old "Youre Old")
 (define young "Youre Young")
 (define res (lambda (age) (if (>= age 40) old young)))
 (print res 60)
 (res 20)
)
