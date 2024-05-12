(ns fib.main)

(defn fib "Take the nth fibonacci number"
  [n]
  (if (or (== n 0) (== n 1))
    n
    (+ (fib (- n 1)) (fib (- n 2)))))

(defn -main [] (println (map fib (range 10))))

;; (-main)
