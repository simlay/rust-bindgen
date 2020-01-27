//#import <Foundation/NSObject.h>

//@protocol Adder <NSObject>
@interface Foo
//@interface Foo: NSObject
- (instancetype) initWithFirstNumber:(int)firstNumber;
@property(nonatomic, readonly) int firstNumber;
@end

@protocol Adder
- (int)addNumber:(int)a toOtherNumber:(int)b;
@end
@interface Bar: Foo <Adder>
@property(nonatomic, readwrite) int someProperty;
- (instancetype) initWithFirstNumber:(int)firstNumber
                     AndSecondNumber:(int)secondNumber;
- (void) frobnicate;
- (int) addNumber:(int)a toOtherNumber:(int)b;
@end
