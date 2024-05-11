(ns mergesort.main)

(defn merge-vecs
  ([x y]
   (cond
     (empty? x) y
     (empty? y) x
     :else (if (< (first x) (first y))
             (cons (first x) (vec (merge-vecs (rest x) y)))
             (cons (first y) (vec (merge-vecs (rest y) x)))))))

(defn merge-sort
  ([x]
   (if (= 1 (count x))
     x
     (let
      [l (map merge-sort (split-at (/ (count x) 2) x))]
       (merge-vecs (first l) (last l))))))

(defn -main [] (println (merge-sort [2 3 5 1 9 6 4 8 7])))

(-main)
