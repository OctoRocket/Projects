(ns thing)


(defn greet [] "Hello")
;; (def greet (fn [] "Hello"))
;; (def greet #(str "Hello"))
(greet)


(defn greeting 
  ([] (greeting "World"))
  ([x] (greeting "Hello" x))
  ([x y] (str x ", " y "!")))
(greeting)
(greeting "Clojure")
(greeting "Good morning" "Clorjure")
(assert (= "Hello, World!" (greeting)))
(assert (= "Hello, Clojure!" (greeting "Clojure")))
(assert (= "Good morning, Clojure!" (greeting "Good morning" "Clojure")))


(defn do-nothing [x] x)
(do-nothing "Helo")

(defn always-thing [& x] 100)
(always-thing 12)

(defn make-thingy [x] (fn [& y] x))
(make-thingy 10)
((make-thingy 10) 19)

(defn triplicate [f] (f) (f) (f))
(triplicate (make-thingy 19))

(defn opposite [f] (fn [& args] (not (apply f args))))
((opposite =) 1 1 1)
((opposite =) 1 1 0)

(defn triplicate2 [f & args] (triplicate (fn [] apply f args)))
(triplicate2 make-thingy 10 11 12)

(Math/cos Math/PI)
(defn somex [x] (+ (Math/pow (Math/sin x) 2) (Math/pow (Math/cos x) 2)))
(somex 30)

(defn http-get [url] (slurp (.openStream (Java.net.URL. url))))