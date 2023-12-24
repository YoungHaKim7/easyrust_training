fun main() {
    val numbers = mutableListOf("one", "two", "three", "four") 
    val mutableIterator = numbers.iterator()
    println("Before removal: $numbers")


    mutableIterator.next()
    mutableIterator.remove()    
    println("After removal: $numbers")
}
