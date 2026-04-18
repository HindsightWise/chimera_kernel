#include "kuzu/include/kuzu_rs.h"
#include <algorithm>
#include <array>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <exception>
#include <initializer_list>
#include <iterator>
#include <memory>
#include <new>
#include <stdexcept>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>
#if __cplusplus >= 201703L
#include <string_view>
#endif
#if __cplusplus >= 202002L
#include <ranges>
#endif

#ifdef __GNUC__
#pragma GCC diagnostic ignored "-Wmissing-declarations"
#pragma GCC diagnostic ignored "-Wshadow"
#ifdef __clang__
#pragma clang diagnostic ignored "-Wdollar-in-identifier-extension"
#endif // __clang__
#endif // __GNUC__

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE1_PANIC
#define CXXBRIDGE1_PANIC
template <typename Exception>
void panic [[noreturn]] (const char *msg);
#endif // CXXBRIDGE1_PANIC

struct unsafe_bitcopy_t;

namespace {
template <typename T>
class impl;
} // namespace

class Opaque;

template <typename T>
::std::size_t size_of();
template <typename T>
::std::size_t align_of();

#ifndef CXXBRIDGE1_RUST_STRING
#define CXXBRIDGE1_RUST_STRING
class String final {
public:
  String() noexcept;
  String(const String &) noexcept;
  String(String &&) noexcept;
  ~String() noexcept;

  String(const std::string &);
  String(const char *);
  String(const char *, std::size_t);
  String(const char16_t *);
  String(const char16_t *, std::size_t);
#ifdef __cpp_char8_t
  String(const char8_t *s);
  String(const char8_t *s, std::size_t len);
#endif

  static String lossy(const std::string &) noexcept;
  static String lossy(const char *) noexcept;
  static String lossy(const char *, std::size_t) noexcept;
  static String lossy(const char16_t *) noexcept;
  static String lossy(const char16_t *, std::size_t) noexcept;

  String &operator=(const String &) & noexcept;
  String &operator=(String &&) & noexcept;

  explicit operator std::string() const;

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  const char *c_str() noexcept;

  std::size_t capacity() const noexcept;
  void reserve(size_t new_cap) noexcept;

  using iterator = char *;
  iterator begin() noexcept;
  iterator end() noexcept;

  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const String &) const noexcept;
  bool operator!=(const String &) const noexcept;
  bool operator<(const String &) const noexcept;
  bool operator<=(const String &) const noexcept;
  bool operator>(const String &) const noexcept;
  bool operator>=(const String &) const noexcept;

  void swap(String &) noexcept;

  String(unsafe_bitcopy_t, const String &) noexcept;

private:
  struct lossy_t;
  String(lossy_t, const char *, std::size_t) noexcept;
  String(lossy_t, const char16_t *, std::size_t) noexcept;
  friend void swap(String &lhs, String &rhs) noexcept { lhs.swap(rhs); }

  std::array<std::uintptr_t, 3> repr;
};
#endif // CXXBRIDGE1_RUST_STRING

#ifndef CXXBRIDGE1_RUST_STR
#define CXXBRIDGE1_RUST_STR
class Str final {
public:
  Str() noexcept;
  Str(const String &) noexcept;
  Str(const std::string &);
  Str(const char *);
  Str(const char *, std::size_t);

  Str &operator=(const Str &) & noexcept = default;

  explicit operator std::string() const;
#if __cplusplus >= 201703L
  explicit operator std::string_view() const;
#endif

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  Str(const Str &) noexcept = default;
  ~Str() noexcept = default;

  using iterator = const char *;
  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const Str &) const noexcept;
  bool operator!=(const Str &) const noexcept;
  bool operator<(const Str &) const noexcept;
  bool operator<=(const Str &) const noexcept;
  bool operator>(const Str &) const noexcept;
  bool operator>=(const Str &) const noexcept;

  void swap(Str &) noexcept;

private:
  class uninit;
  Str(uninit) noexcept;
  friend impl<Str>;

  std::array<std::uintptr_t, 2> repr;
};
#endif // CXXBRIDGE1_RUST_STR

#ifndef CXXBRIDGE1_RUST_SLICE
#define CXXBRIDGE1_RUST_SLICE
namespace detail {
template <bool>
struct copy_assignable_if {};

template <>
struct copy_assignable_if<false> {
  copy_assignable_if() noexcept = default;
  copy_assignable_if(const copy_assignable_if &) noexcept = default;
  copy_assignable_if &operator=(const copy_assignable_if &) & noexcept = delete;
  copy_assignable_if &operator=(copy_assignable_if &&) & noexcept = default;
};
} // namespace detail

template <typename T>
class Slice final
    : private detail::copy_assignable_if<std::is_const<T>::value> {
public:
  using value_type = T;

  Slice() noexcept;
  Slice(T *, std::size_t count) noexcept;

  template <typename C>
  explicit Slice(C &c) : Slice(c.data(), c.size()) {}

  Slice &operator=(const Slice<T> &) & noexcept = default;
  Slice &operator=(Slice<T> &&) & noexcept = default;

  T *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  T &operator[](std::size_t n) const noexcept;
  T &at(std::size_t n) const;
  T &front() const noexcept;
  T &back() const noexcept;

  Slice(const Slice<T> &) noexcept = default;
  ~Slice() noexcept = default;

  class iterator;
  iterator begin() const noexcept;
  iterator end() const noexcept;

  void swap(Slice &) noexcept;

private:
  class uninit;
  Slice(uninit) noexcept;
  friend impl<Slice>;
  friend void sliceInit(void *, const void *, std::size_t) noexcept;
  friend void *slicePtr(const void *) noexcept;
  friend std::size_t sliceLen(const void *) noexcept;

  std::array<std::uintptr_t, 2> repr;
};

#ifdef __cpp_deduction_guides
template <typename C>
explicit Slice(C &c)
    -> Slice<std::remove_reference_t<decltype(*std::declval<C>().data())>>;
#endif // __cpp_deduction_guides

template <typename T>
class Slice<T>::iterator final {
public:
#if __cplusplus >= 202002L
  using iterator_category = std::contiguous_iterator_tag;
#else
  using iterator_category = std::random_access_iterator_tag;
#endif
  using value_type = T;
  using difference_type = std::ptrdiff_t;
  using pointer = typename std::add_pointer<T>::type;
  using reference = typename std::add_lvalue_reference<T>::type;

  reference operator*() const noexcept;
  pointer operator->() const noexcept;
  reference operator[](difference_type) const noexcept;

  iterator &operator++() noexcept;
  iterator operator++(int) noexcept;
  iterator &operator--() noexcept;
  iterator operator--(int) noexcept;

  iterator &operator+=(difference_type) noexcept;
  iterator &operator-=(difference_type) noexcept;
  iterator operator+(difference_type) const noexcept;
  friend inline iterator operator+(difference_type lhs, iterator rhs) noexcept {
    return rhs + lhs;
  }
  iterator operator-(difference_type) const noexcept;
  difference_type operator-(const iterator &) const noexcept;

  bool operator==(const iterator &) const noexcept;
  bool operator!=(const iterator &) const noexcept;
  bool operator<(const iterator &) const noexcept;
  bool operator<=(const iterator &) const noexcept;
  bool operator>(const iterator &) const noexcept;
  bool operator>=(const iterator &) const noexcept;

private:
  friend class Slice;
  void *pos;
  std::size_t stride;
};

#if __cplusplus >= 202002L
static_assert(std::ranges::contiguous_range<rust::Slice<const uint8_t>>);
static_assert(std::contiguous_iterator<rust::Slice<const uint8_t>::iterator>);
#endif

template <typename T>
Slice<T>::Slice() noexcept {
  sliceInit(this, reinterpret_cast<void *>(align_of<T>()), 0);
}

template <typename T>
Slice<T>::Slice(T *s, std::size_t count) noexcept {
  assert(s != nullptr || count == 0);
  sliceInit(this,
            s == nullptr && count == 0
                ? reinterpret_cast<void *>(align_of<T>())
                : const_cast<typename std::remove_const<T>::type *>(s),
            count);
}

template <typename T>
T *Slice<T>::data() const noexcept {
  return reinterpret_cast<T *>(slicePtr(this));
}

template <typename T>
std::size_t Slice<T>::size() const noexcept {
  return sliceLen(this);
}

template <typename T>
std::size_t Slice<T>::length() const noexcept {
  return this->size();
}

template <typename T>
bool Slice<T>::empty() const noexcept {
  return this->size() == 0;
}

template <typename T>
T &Slice<T>::operator[](std::size_t n) const noexcept {
  assert(n < this->size());
  auto ptr = static_cast<char *>(slicePtr(this)) + size_of<T>() * n;
  return *reinterpret_cast<T *>(ptr);
}

template <typename T>
T &Slice<T>::at(std::size_t n) const {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Slice index out of range");
  }
  return (*this)[n];
}

template <typename T>
T &Slice<T>::front() const noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
T &Slice<T>::back() const noexcept {
  assert(!this->empty());
  return (*this)[this->size() - 1];
}

template <typename T>
typename Slice<T>::iterator::reference
Slice<T>::iterator::operator*() const noexcept {
  return *static_cast<T *>(this->pos);
}

template <typename T>
typename Slice<T>::iterator::pointer
Slice<T>::iterator::operator->() const noexcept {
  return static_cast<T *>(this->pos);
}

template <typename T>
typename Slice<T>::iterator::reference Slice<T>::iterator::operator[](
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto ptr = static_cast<char *>(this->pos) + this->stride * n;
  return *reinterpret_cast<T *>(ptr);
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator++() noexcept {
  this->pos = static_cast<char *>(this->pos) + this->stride;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator++(int) noexcept {
  auto ret = iterator(*this);
  this->pos = static_cast<char *>(this->pos) + this->stride;
  return ret;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator--() noexcept {
  this->pos = static_cast<char *>(this->pos) - this->stride;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator--(int) noexcept {
  auto ret = iterator(*this);
  this->pos = static_cast<char *>(this->pos) - this->stride;
  return ret;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator+=(
    typename Slice<T>::iterator::difference_type n) noexcept {
  this->pos = static_cast<char *>(this->pos) + this->stride * n;
  return *this;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator-=(
    typename Slice<T>::iterator::difference_type n) noexcept {
  this->pos = static_cast<char *>(this->pos) - this->stride * n;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator+(
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto ret = iterator(*this);
  ret.pos = static_cast<char *>(this->pos) + this->stride * n;
  return ret;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator-(
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto ret = iterator(*this);
  ret.pos = static_cast<char *>(this->pos) - this->stride * n;
  return ret;
}

template <typename T>
typename Slice<T>::iterator::difference_type
Slice<T>::iterator::operator-(const iterator &other) const noexcept {
  auto diff = std::distance(static_cast<char *>(other.pos),
                            static_cast<char *>(this->pos));
  return diff / static_cast<typename Slice<T>::iterator::difference_type>(
                    this->stride);
}

template <typename T>
bool Slice<T>::iterator::operator==(const iterator &other) const noexcept {
  return this->pos == other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator!=(const iterator &other) const noexcept {
  return this->pos != other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator<(const iterator &other) const noexcept {
  return this->pos < other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator<=(const iterator &other) const noexcept {
  return this->pos <= other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator>(const iterator &other) const noexcept {
  return this->pos > other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator>=(const iterator &other) const noexcept {
  return this->pos >= other.pos;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::begin() const noexcept {
  iterator it;
  it.pos = slicePtr(this);
  it.stride = size_of<T>();
  return it;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::end() const noexcept {
  iterator it = this->begin();
  it.pos = static_cast<char *>(it.pos) + it.stride * this->size();
  return it;
}

template <typename T>
void Slice<T>::swap(Slice &rhs) noexcept {
  std::swap(*this, rhs);
}
#endif // CXXBRIDGE1_RUST_SLICE

#ifndef CXXBRIDGE1_RUST_BITCOPY_T
#define CXXBRIDGE1_RUST_BITCOPY_T
struct unsafe_bitcopy_t final {
  explicit unsafe_bitcopy_t() = default;
};
#endif // CXXBRIDGE1_RUST_BITCOPY_T

#ifndef CXXBRIDGE1_RUST_VEC
#define CXXBRIDGE1_RUST_VEC
template <typename T>
class Vec final {
public:
  using value_type = T;

  Vec() noexcept;
  Vec(std::initializer_list<T>);
  Vec(const Vec &);
  Vec(Vec &&) noexcept;
  ~Vec() noexcept;

  Vec &operator=(Vec &&) & noexcept;
  Vec &operator=(const Vec &) &;

  std::size_t size() const noexcept;
  bool empty() const noexcept;
  const T *data() const noexcept;
  T *data() noexcept;
  std::size_t capacity() const noexcept;

  const T &operator[](std::size_t n) const noexcept;
  const T &at(std::size_t n) const;
  const T &front() const noexcept;
  const T &back() const noexcept;

  T &operator[](std::size_t n) noexcept;
  T &at(std::size_t n);
  T &front() noexcept;
  T &back() noexcept;

  void reserve(std::size_t new_cap);
  void push_back(const T &value);
  void push_back(T &&value);
  template <typename... Args>
  void emplace_back(Args &&...args);
  void truncate(std::size_t len);
  void clear();

  using iterator = typename Slice<T>::iterator;
  iterator begin() noexcept;
  iterator end() noexcept;

  using const_iterator = typename Slice<const T>::iterator;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  void swap(Vec &) noexcept;

  Vec(unsafe_bitcopy_t, const Vec &) noexcept;

private:
  void reserve_total(std::size_t new_cap) noexcept;
  void set_len(std::size_t len) noexcept;
  void drop() noexcept;

  friend void swap(Vec &lhs, Vec &rhs) noexcept { lhs.swap(rhs); }

  std::array<std::uintptr_t, 3> repr;
};

template <typename T>
Vec<T>::Vec(std::initializer_list<T> init) : Vec{} {
  this->reserve_total(init.size());
  std::move(init.begin(), init.end(), std::back_inserter(*this));
}

template <typename T>
Vec<T>::Vec(const Vec &other) : Vec() {
  this->reserve_total(other.size());
  std::copy(other.begin(), other.end(), std::back_inserter(*this));
}

template <typename T>
Vec<T>::Vec(Vec &&other) noexcept : repr(other.repr) {
  new (&other) Vec();
}

template <typename T>
Vec<T>::~Vec() noexcept {
  this->drop();
}

template <typename T>
Vec<T> &Vec<T>::operator=(Vec &&other) & noexcept {
  this->drop();
  this->repr = other.repr;
  new (&other) Vec();
  return *this;
}

template <typename T>
Vec<T> &Vec<T>::operator=(const Vec &other) & {
  if (this != &other) {
    this->drop();
    new (this) Vec(other);
  }
  return *this;
}

template <typename T>
bool Vec<T>::empty() const noexcept {
  return this->size() == 0;
}

template <typename T>
T *Vec<T>::data() noexcept {
  return const_cast<T *>(const_cast<const Vec<T> *>(this)->data());
}

template <typename T>
const T &Vec<T>::operator[](std::size_t n) const noexcept {
  assert(n < this->size());
  auto data = reinterpret_cast<const char *>(this->data());
  return *reinterpret_cast<const T *>(data + n * size_of<T>());
}

template <typename T>
const T &Vec<T>::at(std::size_t n) const {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Vec index out of range");
  }
  return (*this)[n];
}

template <typename T>
const T &Vec<T>::front() const noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
const T &Vec<T>::back() const noexcept {
  assert(!this->empty());
  return (*this)[this->size() - 1];
}

template <typename T>
T &Vec<T>::operator[](std::size_t n) noexcept {
  assert(n < this->size());
  auto data = reinterpret_cast<char *>(this->data());
  return *reinterpret_cast<T *>(data + n * size_of<T>());
}

template <typename T>
T &Vec<T>::at(std::size_t n) {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Vec index out of range");
  }
  return (*this)[n];
}

template <typename T>
T &Vec<T>::front() noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
T &Vec<T>::back() noexcept {
  assert(!this->empty());
  return (*this)[this->size() - 1];
}

template <typename T>
void Vec<T>::reserve(std::size_t new_cap) {
  this->reserve_total(new_cap);
}

template <typename T>
void Vec<T>::push_back(const T &value) {
  this->emplace_back(value);
}

template <typename T>
void Vec<T>::push_back(T &&value) {
  this->emplace_back(std::move(value));
}

template <typename T>
template <typename... Args>
void Vec<T>::emplace_back(Args &&...args) {
  auto size = this->size();
  this->reserve_total(size + 1);
  ::new (reinterpret_cast<T *>(reinterpret_cast<char *>(this->data()) +
                               size * size_of<T>()))
      T(std::forward<Args>(args)...);
  this->set_len(size + 1);
}

template <typename T>
void Vec<T>::clear() {
  this->truncate(0);
}

template <typename T>
typename Vec<T>::iterator Vec<T>::begin() noexcept {
  return Slice<T>(this->data(), this->size()).begin();
}

template <typename T>
typename Vec<T>::iterator Vec<T>::end() noexcept {
  return Slice<T>(this->data(), this->size()).end();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::begin() const noexcept {
  return this->cbegin();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::end() const noexcept {
  return this->cend();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::cbegin() const noexcept {
  return Slice<const T>(this->data(), this->size()).begin();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::cend() const noexcept {
  return Slice<const T>(this->data(), this->size()).end();
}

template <typename T>
void Vec<T>::swap(Vec &rhs) noexcept {
  using std::swap;
  swap(this->repr, rhs.repr);
}

template <typename T>
Vec<T>::Vec(unsafe_bitcopy_t, const Vec &bits) noexcept : repr(bits.repr) {}
#endif // CXXBRIDGE1_RUST_VEC

#ifndef CXXBRIDGE1_IS_COMPLETE
#define CXXBRIDGE1_IS_COMPLETE
namespace detail {
namespace {
template <typename T, typename = std::size_t>
struct is_complete : std::false_type {};
template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};
} // namespace
} // namespace detail
#endif // CXXBRIDGE1_IS_COMPLETE

#ifndef CXXBRIDGE1_LAYOUT
#define CXXBRIDGE1_LAYOUT
class layout {
  template <typename T>
  friend std::size_t size_of();
  template <typename T>
  friend std::size_t align_of();
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return T::layout::size();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return sizeof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      size_of() {
    return do_size_of<T>();
  }
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return T::layout::align();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return alignof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      align_of() {
    return do_align_of<T>();
  }
};

template <typename T>
std::size_t size_of() {
  return layout::size_of<T>();
}

template <typename T>
std::size_t align_of() {
  return layout::align_of<T>();
}
#endif // CXXBRIDGE1_LAYOUT

#ifndef CXXBRIDGE1_RELOCATABLE
#define CXXBRIDGE1_RELOCATABLE
namespace detail {
template <typename... Ts>
struct make_void {
  using type = void;
};

template <typename... Ts>
using void_t = typename make_void<Ts...>::type;

template <typename Void, template <typename...> class, typename...>
struct detect : std::false_type {};
template <template <typename...> class T, typename... A>
struct detect<void_t<T<A...>>, T, A...> : std::true_type {};

template <template <typename...> class T, typename... A>
using is_detected = detect<void, T, A...>;

template <typename T>
using detect_IsRelocatable = typename T::IsRelocatable;

template <typename T>
struct get_IsRelocatable
    : std::is_same<typename T::IsRelocatable, std::true_type> {};
} // namespace detail

template <typename T>
struct IsRelocatable
    : std::conditional<
          detail::is_detected<detail::detect_IsRelocatable, T>::value,
          detail::get_IsRelocatable<T>,
          std::integral_constant<
              bool, std::is_trivially_move_constructible<T>::value &&
                        std::is_trivially_destructible<T>::value>>::type {};
#endif // CXXBRIDGE1_RELOCATABLE

namespace repr {
struct PtrLen final {
  void *ptr;
  ::std::size_t len;
};
} // namespace repr

namespace detail {
class Fail final {
  ::rust::repr::PtrLen &throw$;
public:
  Fail(::rust::repr::PtrLen &throw$) noexcept : throw$(throw$) {}
  void operator()(char const *) noexcept;
  void operator()(std::string const &) noexcept;
};
} // namespace detail

namespace {
template <typename T>
void destroy(T *ptr) {
  ptr->~T();
}

template <bool> struct deleter_if {
  template <typename T> void operator()(T *) {}
};
template <> struct deleter_if<true> {
  template <typename T> void operator()(T *ptr) { ptr->~T(); }
};

template <typename T, bool = ::rust::detail::is_complete<T>::value>
struct is_destructible : ::std::false_type {};
template <typename T>
struct is_destructible<T, true> : ::std::is_destructible<T> {};
template <typename T>
struct is_destructible<T[], false> : is_destructible<T> {};
template <typename T, bool = ::rust::is_destructible<T>::value>
struct shared_ptr_if_destructible {
  explicit shared_ptr_if_destructible(typename ::std::shared_ptr<T>::element_type *) {}
};
template <typename T>
struct shared_ptr_if_destructible<T, true> : ::std::shared_ptr<T> {
  using ::std::shared_ptr<T>::shared_ptr;
};

template <typename T, bool = ::std::is_move_constructible<T>::value>
struct if_move_constructible {
  static bool reserve(::std::vector<T> &, ::std::size_t) noexcept {
    return false;
  }
};
template <typename T>
struct if_move_constructible<T, true> {
  static bool reserve(::std::vector<T> &vec, ::std::size_t new_cap) {
    vec.reserve(new_cap);
    return true;
  }
};
} // namespace
} // namespace cxxbridge1

namespace behavior {
class missing {};
missing trycatch(...);

template <typename Try, typename Fail>
static typename ::std::enable_if<::std::is_same<
    decltype(trycatch(::std::declval<Try>(), ::std::declval<Fail>())),
    missing>::value>::type
trycatch(Try &&func, Fail &&fail) noexcept try {
  func();
} catch (::std::exception const &e) {
  fail(e.what());
}
} // namespace behavior
} // namespace rust

namespace kuzu {
  namespace common {
    using LogicalTypeID = ::kuzu::common::LogicalTypeID;
    using LogicalType = ::kuzu::common::LogicalType;
    using Value = ::kuzu::common::Value;
  }
  namespace main {
    using PreparedStatement = ::kuzu::main::PreparedStatement;
    using Database = ::kuzu::main::Database;
    using Connection = ::kuzu::main::Connection;
    using QueryResult = ::kuzu::main::QueryResult;
  }
  namespace processor {
    using FlatTuple = ::kuzu::processor::FlatTuple;
  }
}
namespace kuzu_rs {
  using QueryParams = ::kuzu_rs::QueryParams;
  using ValueListBuilder = ::kuzu_rs::ValueListBuilder;
  using TypeListBuilder = ::kuzu_rs::TypeListBuilder;
}

namespace kuzu {
namespace common {
static_assert(::std::is_enum<LogicalTypeID>::value, "expected enum");
static_assert(sizeof(LogicalTypeID) == sizeof(::std::uint8_t), "incorrect size");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::ANY) == 0, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::NODE) == 10, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::REL) == 11, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::RECURSIVE_REL) == 12, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::SERIAL) == 13, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::BOOL) == 22, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::INT64) == 23, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::INT32) == 24, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::INT16) == 25, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::INT8) == 26, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::UINT64) == 27, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::UINT32) == 28, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::UINT16) == 29, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::UINT8) == 30, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::INT128) == 31, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::DOUBLE) == 32, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::FLOAT) == 33, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::DATE) == 34, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::TIMESTAMP) == 35, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::TIMESTAMP_SEC) == 36, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::TIMESTAMP_MS) == 37, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::TIMESTAMP_NS) == 38, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::TIMESTAMP_TZ) == 39, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::INTERVAL) == 40, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::FIXED_LIST) == 41, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::INTERNAL_ID) == 42, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::STRING) == 50, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::BLOB) == 51, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::VAR_LIST) == 52, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::STRUCT) == 53, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::MAP) == 54, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::UNION) == 55, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::RDF_VARIANT) == 56, "disagrees with the value in #[cxx::bridge]");
static_assert(static_cast<::std::uint8_t>(LogicalTypeID::UUID) == 58, "disagrees with the value in #[cxx::bridge]");
} // namespace common
} // namespace kuzu

static_assert(
    ::rust::IsRelocatable<::std::string_view>::value,
    "type std::string_view should be trivially move constructible and trivially destructible in C++ to be used as an argument of `new_database`, `prepare` or return value of `string_view_from_str` in Rust");

namespace kuzu_rs {
extern "C" {
void kuzu_rs$cxxbridge1$194$string_view_from_str(::rust::Str s, ::std::string_view *return$) noexcept {
  ::std::string_view (*string_view_from_str$)(::rust::Str) = ::kuzu_rs::string_view_from_str;
  new (return$) ::std::string_view(string_view_from_str$(s));
}
} // extern "C"
} // namespace kuzu_rs

namespace kuzu {
namespace main {
extern "C" {
bool kuzu$main$cxxbridge1$194$PreparedStatement$isSuccess(::kuzu::main::PreparedStatement const &self) noexcept {
  bool (::kuzu::main::PreparedStatement::*isSuccess$)() const = &::kuzu::main::PreparedStatement::isSuccess;
  return (self.*isSuccess$)();
}
} // extern "C"
} // namespace main
} // namespace kuzu

namespace kuzu_rs {
extern "C" {
void kuzu_rs$cxxbridge1$194$prepared_statement_error_message(::kuzu::main::PreparedStatement const &statement, ::rust::String *return$) noexcept {
  ::rust::String (*prepared_statement_error_message$)(::kuzu::main::PreparedStatement const &) = ::kuzu_rs::prepared_statement_error_message;
  new (return$) ::rust::String(prepared_statement_error_message$(statement));
}

void kuzu_rs$cxxbridge1$194$QueryParams$insert(::kuzu_rs::QueryParams &self, ::rust::Str key, ::kuzu::common::Value *value) noexcept {
  void (::kuzu_rs::QueryParams::*insert$)(::rust::Str, ::std::unique_ptr<::kuzu::common::Value>) = &::kuzu_rs::QueryParams::insert;
  (self.*insert$)(key, ::std::unique_ptr<::kuzu::common::Value>(value));
}

::kuzu_rs::QueryParams *kuzu_rs$cxxbridge1$194$new_params() noexcept {
  ::std::unique_ptr<::kuzu_rs::QueryParams> (*new_params$)() = ::kuzu_rs::new_params;
  return new_params$().release();
}

::rust::repr::PtrLen kuzu_rs$cxxbridge1$194$new_database(::std::string_view *databasePath, ::std::uint64_t bufferPoolSize, ::std::uint64_t maxNumThreads, bool enableCompression, bool readOnly, ::std::uint64_t maxDBSize, ::kuzu::main::Database **return$) noexcept {
  ::std::unique_ptr<::kuzu::main::Database> (*new_database$)(::std::string_view, ::std::uint64_t, ::std::uint64_t, bool, bool, ::std::uint64_t) = ::kuzu_rs::new_database;
  ::rust::repr::PtrLen throw$;
  ::rust::behavior::trycatch(
      [&] {
        new (return$) ::kuzu::main::Database *(new_database$(::std::move(*databasePath), bufferPoolSize, maxNumThreads, enableCompression, readOnly, maxDBSize).release());
        throw$.ptr = nullptr;
      },
      ::rust::detail::Fail(throw$));
  return throw$;
}

void kuzu_rs$cxxbridge1$194$database_set_logging_level(::kuzu::main::Database &database, ::std::string const &level) noexcept {
  void (*database_set_logging_level$)(::kuzu::main::Database &, ::std::string const &) = ::kuzu_rs::database_set_logging_level;
  database_set_logging_level$(database, level);
}

::rust::repr::PtrLen kuzu_rs$cxxbridge1$194$database_connect(::kuzu::main::Database &database, ::kuzu::main::Connection **return$) noexcept {
  ::std::unique_ptr<::kuzu::main::Connection> (*database_connect$)(::kuzu::main::Database &) = ::kuzu_rs::database_connect;
  ::rust::repr::PtrLen throw$;
  ::rust::behavior::trycatch(
      [&] {
        new (return$) ::kuzu::main::Connection *(database_connect$(database).release());
        throw$.ptr = nullptr;
      },
      ::rust::detail::Fail(throw$));
  return throw$;
}
} // extern "C"
} // namespace kuzu_rs

namespace kuzu {
namespace main {
extern "C" {
::rust::repr::PtrLen kuzu$main$cxxbridge1$194$Connection$prepare(::kuzu::main::Connection &self, ::std::string_view *query, ::kuzu::main::PreparedStatement **return$) noexcept {
  ::std::unique_ptr<::kuzu::main::PreparedStatement> (::kuzu::main::Connection::*prepare$)(::std::string_view) = &::kuzu::main::Connection::prepare;
  ::rust::repr::PtrLen throw$;
  ::rust::behavior::trycatch(
      [&] {
        new (return$) ::kuzu::main::PreparedStatement *((self.*prepare$)(::std::move(*query)).release());
        throw$.ptr = nullptr;
      },
      ::rust::detail::Fail(throw$));
  return throw$;
}
} // extern "C"
} // namespace main
} // namespace kuzu

namespace kuzu_rs {
extern "C" {
::rust::repr::PtrLen kuzu_rs$cxxbridge1$194$connection_execute(::kuzu::main::Connection &connection, ::kuzu::main::PreparedStatement &query, ::kuzu_rs::QueryParams *params, ::kuzu::main::QueryResult **return$) noexcept {
  ::std::unique_ptr<::kuzu::main::QueryResult> (*connection_execute$)(::kuzu::main::Connection &, ::kuzu::main::PreparedStatement &, ::std::unique_ptr<::kuzu_rs::QueryParams>) = ::kuzu_rs::connection_execute;
  ::rust::repr::PtrLen throw$;
  ::rust::behavior::trycatch(
      [&] {
        new (return$) ::kuzu::main::QueryResult *(connection_execute$(connection, query, ::std::unique_ptr<::kuzu_rs::QueryParams>(params)).release());
        throw$.ptr = nullptr;
      },
      ::rust::detail::Fail(throw$));
  return throw$;
}
} // extern "C"
} // namespace kuzu_rs

namespace kuzu {
namespace main {
extern "C" {
::std::uint64_t kuzu$main$cxxbridge1$194$Connection$getMaxNumThreadForExec(::kuzu::main::Connection &self) noexcept {
  ::std::uint64_t (::kuzu::main::Connection::*getMaxNumThreadForExec$)() = &::kuzu::main::Connection::getMaxNumThreadForExec;
  return (self.*getMaxNumThreadForExec$)();
}

void kuzu$main$cxxbridge1$194$Connection$setMaxNumThreadForExec(::kuzu::main::Connection &self, ::std::uint64_t num_threads) noexcept {
  void (::kuzu::main::Connection::*setMaxNumThreadForExec$)(::std::uint64_t) = &::kuzu::main::Connection::setMaxNumThreadForExec;
  (self.*setMaxNumThreadForExec$)(num_threads);
}

::rust::repr::PtrLen kuzu$main$cxxbridge1$194$Connection$interrupt(::kuzu::main::Connection &self) noexcept {
  void (::kuzu::main::Connection::*interrupt$)() = &::kuzu::main::Connection::interrupt;
  ::rust::repr::PtrLen throw$;
  ::rust::behavior::trycatch(
      [&] {
        (self.*interrupt$)();
        throw$.ptr = nullptr;
      },
      ::rust::detail::Fail(throw$));
  return throw$;
}

void kuzu$main$cxxbridge1$194$Connection$setQueryTimeOut(::kuzu::main::Connection &self, ::std::uint64_t timeout_ms) noexcept {
  void (::kuzu::main::Connection::*setQueryTimeOut$)(::std::uint64_t) = &::kuzu::main::Connection::setQueryTimeOut;
  (self.*setQueryTimeOut$)(timeout_ms);
}
} // extern "C"
} // namespace main
} // namespace kuzu

namespace kuzu_rs {
extern "C" {
void kuzu_rs$cxxbridge1$194$query_result_to_string(::kuzu::main::QueryResult &query_result, ::rust::String *return$) noexcept {
  ::rust::String (*query_result_to_string$)(::kuzu::main::QueryResult &) = ::kuzu_rs::query_result_to_string;
  new (return$) ::rust::String(query_result_to_string$(query_result));
}
} // extern "C"
} // namespace kuzu_rs

namespace kuzu {
namespace main {
extern "C" {
bool kuzu$main$cxxbridge1$194$QueryResult$isSuccess(::kuzu::main::QueryResult const &self) noexcept {
  bool (::kuzu::main::QueryResult::*isSuccess$)() const = &::kuzu::main::QueryResult::isSuccess;
  return (self.*isSuccess$)();
}
} // extern "C"
} // namespace main
} // namespace kuzu

namespace kuzu_rs {
extern "C" {
void kuzu_rs$cxxbridge1$194$query_result_get_error_message(::kuzu::main::QueryResult const &query_result, ::rust::String *return$) noexcept {
  ::rust::String (*query_result_get_error_message$)(::kuzu::main::QueryResult const &) = ::kuzu_rs::query_result_get_error_message;
  new (return$) ::rust::String(query_result_get_error_message$(query_result));
}
} // extern "C"
} // namespace kuzu_rs

namespace kuzu {
namespace main {
extern "C" {
bool kuzu$main$cxxbridge1$194$QueryResult$hasNext(::kuzu::main::QueryResult const &self) noexcept {
  bool (::kuzu::main::QueryResult::*hasNext$)() const = &::kuzu::main::QueryResult::hasNext;
  return (self.*hasNext$)();
}

void kuzu$main$cxxbridge1$194$QueryResult$getNext(::kuzu::main::QueryResult &self, ::std::shared_ptr<::kuzu::processor::FlatTuple> *return$) noexcept {
  ::std::shared_ptr<::kuzu::processor::FlatTuple> (::kuzu::main::QueryResult::*getNext$)() = &::kuzu::main::QueryResult::getNext;
  new (return$) ::std::shared_ptr<::kuzu::processor::FlatTuple>((self.*getNext$)());
}
} // extern "C"
} // namespace main
} // namespace kuzu

namespace kuzu_rs {
extern "C" {
double kuzu_rs$cxxbridge1$194$query_result_get_compiling_time(::kuzu::main::QueryResult const &result) noexcept {
  double (*query_result_get_compiling_time$)(::kuzu::main::QueryResult const &) = ::kuzu_rs::query_result_get_compiling_time;
  return query_result_get_compiling_time$(result);
}

double kuzu_rs$cxxbridge1$194$query_result_get_execution_time(::kuzu::main::QueryResult const &result) noexcept {
  double (*query_result_get_execution_time$)(::kuzu::main::QueryResult const &) = ::kuzu_rs::query_result_get_execution_time;
  return query_result_get_execution_time$(result);
}
} // extern "C"
} // namespace kuzu_rs

namespace kuzu {
namespace main {
extern "C" {
::std::size_t kuzu$main$cxxbridge1$194$QueryResult$getNumColumns(::kuzu::main::QueryResult const &self) noexcept {
  ::std::size_t (::kuzu::main::QueryResult::*getNumColumns$)() const = &::kuzu::main::QueryResult::getNumColumns;
  return (self.*getNumColumns$)();
}

::std::uint64_t kuzu$main$cxxbridge1$194$QueryResult$getNumTuples(::kuzu::main::QueryResult const &self) noexcept {
  ::std::uint64_t (::kuzu::main::QueryResult::*getNumTuples$)() const = &::kuzu::main::QueryResult::getNumTuples;
  return (self.*getNumTuples$)();
}
} // extern "C"
} // namespace main
} // namespace kuzu

namespace kuzu_rs {
extern "C" {
::std::vector<::kuzu::common::LogicalType> *kuzu_rs$cxxbridge1$194$query_result_column_data_types(::kuzu::main::QueryResult const &query_result) noexcept {
  ::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>> (*query_result_column_data_types$)(::kuzu::main::QueryResult const &) = ::kuzu_rs::query_result_column_data_types;
  return query_result_column_data_types$(query_result).release();
}

void kuzu_rs$cxxbridge1$194$query_result_column_names(::kuzu::main::QueryResult const &query_result, ::rust::Vec<::rust::String> *return$) noexcept {
  ::rust::Vec<::rust::String> (*query_result_column_names$)(::kuzu::main::QueryResult const &) = ::kuzu_rs::query_result_column_names;
  new (return$) ::rust::Vec<::rust::String>(query_result_column_names$(query_result));
}
} // extern "C"
} // namespace kuzu_rs

namespace kuzu {
namespace processor {
extern "C" {
::std::uint32_t kuzu$processor$cxxbridge1$194$FlatTuple$len(::kuzu::processor::FlatTuple const &self) noexcept {
  ::std::uint32_t (::kuzu::processor::FlatTuple::*len$)() const = &::kuzu::processor::FlatTuple::len;
  return (self.*len$)();
}
} // extern "C"
} // namespace processor
} // namespace kuzu

namespace kuzu_rs {
extern "C" {
void kuzu_rs$cxxbridge1$194$flat_tuple_get_value(::kuzu::processor::FlatTuple const &tuple, ::std::uint32_t index, ::kuzu::common::Value const **return$) noexcept {
  ::kuzu::common::Value const &(*flat_tuple_get_value$)(::kuzu::processor::FlatTuple const &, ::std::uint32_t) = ::kuzu_rs::flat_tuple_get_value;
  new (return$) ::kuzu::common::Value const *(&flat_tuple_get_value$(tuple, index));
}
} // extern "C"
} // namespace kuzu_rs

namespace kuzu {
namespace common {
extern "C" {
::kuzu::common::LogicalTypeID kuzu$common$cxxbridge1$194$LogicalType$getLogicalTypeID(::kuzu::common::LogicalType const &self) noexcept {
  ::kuzu::common::LogicalTypeID (::kuzu::common::LogicalType::*getLogicalTypeID$)() const = &::kuzu::common::LogicalType::getLogicalTypeID;
  return (self.*getLogicalTypeID$)();
}
} // extern "C"
} // namespace common
} // namespace kuzu

namespace kuzu_rs {
extern "C" {
::kuzu::common::LogicalType *kuzu_rs$cxxbridge1$194$create_logical_type(::kuzu::common::LogicalTypeID id) noexcept {
  ::std::unique_ptr<::kuzu::common::LogicalType> (*create_logical_type$)(::kuzu::common::LogicalTypeID) = ::kuzu_rs::create_logical_type;
  return create_logical_type$(id).release();
}

::kuzu::common::LogicalType *kuzu_rs$cxxbridge1$194$create_logical_type_var_list(::kuzu::common::LogicalType *child_type) noexcept {
  ::std::unique_ptr<::kuzu::common::LogicalType> (*create_logical_type_var_list$)(::std::unique_ptr<::kuzu::common::LogicalType>) = ::kuzu_rs::create_logical_type_var_list;
  return create_logical_type_var_list$(::std::unique_ptr<::kuzu::common::LogicalType>(child_type)).release();
}

::kuzu::common::LogicalType *kuzu_rs$cxxbridge1$194$create_logical_type_fixed_list(::kuzu::common::LogicalType *child_type, ::std::uint64_t num_elements) noexcept {
  ::std::unique_ptr<::kuzu::common::LogicalType> (*create_logical_type_fixed_list$)(::std::unique_ptr<::kuzu::common::LogicalType>, ::std::uint64_t) = ::kuzu_rs::create_logical_type_fixed_list;
  return create_logical_type_fixed_list$(::std::unique_ptr<::kuzu::common::LogicalType>(child_type), num_elements).release();
}

::kuzu::common::LogicalType *kuzu_rs$cxxbridge1$194$create_logical_type_struct(::rust::Vec<::rust::String> const &field_names, ::kuzu_rs::TypeListBuilder *types) noexcept {
  ::std::unique_ptr<::kuzu::common::LogicalType> (*create_logical_type_struct$)(::rust::Vec<::rust::String> const &, ::std::unique_ptr<::kuzu_rs::TypeListBuilder>) = ::kuzu_rs::create_logical_type_struct;
  return create_logical_type_struct$(field_names, ::std::unique_ptr<::kuzu_rs::TypeListBuilder>(types)).release();
}

::kuzu::common::LogicalType *kuzu_rs$cxxbridge1$194$create_logical_type_union(::rust::Vec<::rust::String> const &field_names, ::kuzu_rs::TypeListBuilder *types) noexcept {
  ::std::unique_ptr<::kuzu::common::LogicalType> (*create_logical_type_union$)(::rust::Vec<::rust::String> const &, ::std::unique_ptr<::kuzu_rs::TypeListBuilder>) = ::kuzu_rs::create_logical_type_union;
  return create_logical_type_union$(field_names, ::std::unique_ptr<::kuzu_rs::TypeListBuilder>(types)).release();
}

::kuzu::common::LogicalType *kuzu_rs$cxxbridge1$194$create_logical_type_map(::kuzu::common::LogicalType *keyType, ::kuzu::common::LogicalType *valueType) noexcept {
  ::std::unique_ptr<::kuzu::common::LogicalType> (*create_logical_type_map$)(::std::unique_ptr<::kuzu::common::LogicalType>, ::std::unique_ptr<::kuzu::common::LogicalType>) = ::kuzu_rs::create_logical_type_map;
  return create_logical_type_map$(::std::unique_ptr<::kuzu::common::LogicalType>(keyType), ::std::unique_ptr<::kuzu::common::LogicalType>(valueType)).release();
}

void kuzu_rs$cxxbridge1$194$logical_type_get_var_list_child_type(::kuzu::common::LogicalType const &value, ::kuzu::common::LogicalType const **return$) noexcept {
  ::kuzu::common::LogicalType const &(*logical_type_get_var_list_child_type$)(::kuzu::common::LogicalType const &) = ::kuzu_rs::logical_type_get_var_list_child_type;
  new (return$) ::kuzu::common::LogicalType const *(&logical_type_get_var_list_child_type$(value));
}

void kuzu_rs$cxxbridge1$194$logical_type_get_fixed_list_child_type(::kuzu::common::LogicalType const &value, ::kuzu::common::LogicalType const **return$) noexcept {
  ::kuzu::common::LogicalType const &(*logical_type_get_fixed_list_child_type$)(::kuzu::common::LogicalType const &) = ::kuzu_rs::logical_type_get_fixed_list_child_type;
  new (return$) ::kuzu::common::LogicalType const *(&logical_type_get_fixed_list_child_type$(value));
}

::std::uint64_t kuzu_rs$cxxbridge1$194$logical_type_get_fixed_list_num_elements(::kuzu::common::LogicalType const &value) noexcept {
  ::std::uint64_t (*logical_type_get_fixed_list_num_elements$)(::kuzu::common::LogicalType const &) = ::kuzu_rs::logical_type_get_fixed_list_num_elements;
  return logical_type_get_fixed_list_num_elements$(value);
}

void kuzu_rs$cxxbridge1$194$logical_type_get_struct_field_names(::kuzu::common::LogicalType const &value, ::rust::Vec<::rust::String> *return$) noexcept {
  ::rust::Vec<::rust::String> (*logical_type_get_struct_field_names$)(::kuzu::common::LogicalType const &) = ::kuzu_rs::logical_type_get_struct_field_names;
  new (return$) ::rust::Vec<::rust::String>(logical_type_get_struct_field_names$(value));
}

::std::vector<::kuzu::common::LogicalType> *kuzu_rs$cxxbridge1$194$logical_type_get_struct_field_types(::kuzu::common::LogicalType const &value) noexcept {
  ::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>> (*logical_type_get_struct_field_types$)(::kuzu::common::LogicalType const &) = ::kuzu_rs::logical_type_get_struct_field_types;
  return logical_type_get_struct_field_types$(value).release();
}

::kuzu::common::LogicalType *kuzu_rs$cxxbridge1$194$create_logical_type_rdf_variant() noexcept {
  ::std::unique_ptr<::kuzu::common::LogicalType> (*create_logical_type_rdf_variant$)() = ::kuzu_rs::create_logical_type_rdf_variant;
  return create_logical_type_rdf_variant$().release();
}

void kuzu_rs$cxxbridge1$194$ValueListBuilder$insert(::kuzu_rs::ValueListBuilder &self, ::kuzu::common::Value *value) noexcept {
  void (::kuzu_rs::ValueListBuilder::*insert$)(::std::unique_ptr<::kuzu::common::Value>) = &::kuzu_rs::ValueListBuilder::insert;
  (self.*insert$)(::std::unique_ptr<::kuzu::common::Value>(value));
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$get_list_value(::kuzu::common::LogicalType *typ, ::kuzu_rs::ValueListBuilder *value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*get_list_value$)(::std::unique_ptr<::kuzu::common::LogicalType>, ::std::unique_ptr<::kuzu_rs::ValueListBuilder>) = ::kuzu_rs::get_list_value;
  return get_list_value$(::std::unique_ptr<::kuzu::common::LogicalType>(typ), ::std::unique_ptr<::kuzu_rs::ValueListBuilder>(value)).release();
}

::kuzu_rs::ValueListBuilder *kuzu_rs$cxxbridge1$194$create_list() noexcept {
  ::std::unique_ptr<::kuzu_rs::ValueListBuilder> (*create_list$)() = ::kuzu_rs::create_list;
  return create_list$().release();
}

void kuzu_rs$cxxbridge1$194$TypeListBuilder$insert(::kuzu_rs::TypeListBuilder &self, ::kuzu::common::LogicalType *typ) noexcept {
  void (::kuzu_rs::TypeListBuilder::*insert$)(::std::unique_ptr<::kuzu::common::LogicalType>) = &::kuzu_rs::TypeListBuilder::insert;
  (self.*insert$)(::std::unique_ptr<::kuzu::common::LogicalType>(typ));
}

::kuzu_rs::TypeListBuilder *kuzu_rs$cxxbridge1$194$create_type_list() noexcept {
  ::std::unique_ptr<::kuzu_rs::TypeListBuilder> (*create_type_list$)() = ::kuzu_rs::create_type_list;
  return create_type_list$().release();
}

void kuzu_rs$cxxbridge1$194$value_to_string(::kuzu::common::Value const &node_value, ::rust::String *return$) noexcept {
  ::rust::String (*value_to_string$)(::kuzu::common::Value const &) = ::kuzu_rs::value_to_string;
  new (return$) ::rust::String(value_to_string$(node_value));
}

bool kuzu_rs$cxxbridge1$194$Value$get_value_bool(::kuzu::common::Value const &self) noexcept {
  bool (::kuzu::common::Value::*get_value_bool$)() const = &::kuzu::common::Value::getValue;
  return (self.*get_value_bool$)();
}

::std::int8_t kuzu_rs$cxxbridge1$194$Value$get_value_i8(::kuzu::common::Value const &self) noexcept {
  ::std::int8_t (::kuzu::common::Value::*get_value_i8$)() const = &::kuzu::common::Value::getValue;
  return (self.*get_value_i8$)();
}

::std::int16_t kuzu_rs$cxxbridge1$194$Value$get_value_i16(::kuzu::common::Value const &self) noexcept {
  ::std::int16_t (::kuzu::common::Value::*get_value_i16$)() const = &::kuzu::common::Value::getValue;
  return (self.*get_value_i16$)();
}

::std::int32_t kuzu_rs$cxxbridge1$194$Value$get_value_i32(::kuzu::common::Value const &self) noexcept {
  ::std::int32_t (::kuzu::common::Value::*get_value_i32$)() const = &::kuzu::common::Value::getValue;
  return (self.*get_value_i32$)();
}

::std::int64_t kuzu_rs$cxxbridge1$194$Value$get_value_i64(::kuzu::common::Value const &self) noexcept {
  ::std::int64_t (::kuzu::common::Value::*get_value_i64$)() const = &::kuzu::common::Value::getValue;
  return (self.*get_value_i64$)();
}

::std::uint8_t kuzu_rs$cxxbridge1$194$Value$get_value_u8(::kuzu::common::Value const &self) noexcept {
  ::std::uint8_t (::kuzu::common::Value::*get_value_u8$)() const = &::kuzu::common::Value::getValue;
  return (self.*get_value_u8$)();
}

::std::uint16_t kuzu_rs$cxxbridge1$194$Value$get_value_u16(::kuzu::common::Value const &self) noexcept {
  ::std::uint16_t (::kuzu::common::Value::*get_value_u16$)() const = &::kuzu::common::Value::getValue;
  return (self.*get_value_u16$)();
}

::std::uint32_t kuzu_rs$cxxbridge1$194$Value$get_value_u32(::kuzu::common::Value const &self) noexcept {
  ::std::uint32_t (::kuzu::common::Value::*get_value_u32$)() const = &::kuzu::common::Value::getValue;
  return (self.*get_value_u32$)();
}

::std::uint64_t kuzu_rs$cxxbridge1$194$Value$get_value_u64(::kuzu::common::Value const &self) noexcept {
  ::std::uint64_t (::kuzu::common::Value::*get_value_u64$)() const = &::kuzu::common::Value::getValue;
  return (self.*get_value_u64$)();
}

float kuzu_rs$cxxbridge1$194$Value$get_value_float(::kuzu::common::Value const &self) noexcept {
  float (::kuzu::common::Value::*get_value_float$)() const = &::kuzu::common::Value::getValue;
  return (self.*get_value_float$)();
}

double kuzu_rs$cxxbridge1$194$Value$get_value_double(::kuzu::common::Value const &self) noexcept {
  double (::kuzu::common::Value::*get_value_double$)() const = &::kuzu::common::Value::getValue;
  return (self.*get_value_double$)();
}

void kuzu_rs$cxxbridge1$194$value_get_string(::kuzu::common::Value const &value, ::std::string const **return$) noexcept {
  ::std::string const &(*value_get_string$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_string;
  new (return$) ::std::string const *(&value_get_string$(value));
}

::std::int64_t kuzu_rs$cxxbridge1$194$value_get_interval_secs(::kuzu::common::Value const &value) noexcept {
  ::std::int64_t (*value_get_interval_secs$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_interval_secs;
  return value_get_interval_secs$(value);
}

::std::int32_t kuzu_rs$cxxbridge1$194$value_get_interval_micros(::kuzu::common::Value const &value) noexcept {
  ::std::int32_t (*value_get_interval_micros$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_interval_micros;
  return value_get_interval_micros$(value);
}

::std::int64_t kuzu_rs$cxxbridge1$194$value_get_timestamp_micros(::kuzu::common::Value const &value) noexcept {
  ::std::int64_t (*value_get_timestamp_micros$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_timestamp_micros;
  return value_get_timestamp_micros$(value);
}

::std::int64_t kuzu_rs$cxxbridge1$194$value_get_timestamp_ns(::kuzu::common::Value const &value) noexcept {
  ::std::int64_t (*value_get_timestamp_ns$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_timestamp_ns;
  return value_get_timestamp_ns$(value);
}

::std::int64_t kuzu_rs$cxxbridge1$194$value_get_timestamp_ms(::kuzu::common::Value const &value) noexcept {
  ::std::int64_t (*value_get_timestamp_ms$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_timestamp_ms;
  return value_get_timestamp_ms$(value);
}

::std::int64_t kuzu_rs$cxxbridge1$194$value_get_timestamp_sec(::kuzu::common::Value const &value) noexcept {
  ::std::int64_t (*value_get_timestamp_sec$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_timestamp_sec;
  return value_get_timestamp_sec$(value);
}

::std::int64_t kuzu_rs$cxxbridge1$194$value_get_timestamp_tz(::kuzu::common::Value const &value) noexcept {
  ::std::int64_t (*value_get_timestamp_tz$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_timestamp_tz;
  return value_get_timestamp_tz$(value);
}

::std::int32_t kuzu_rs$cxxbridge1$194$value_get_date_days(::kuzu::common::Value const &value) noexcept {
  ::std::int32_t (*value_get_date_days$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_date_days;
  return value_get_date_days$(value);
}

void kuzu_rs$cxxbridge1$194$value_get_int128_t(::kuzu::common::Value const &value, ::std::array<::std::uint64_t, 2> *return$) noexcept {
  ::std::array<::std::uint64_t, 2> (*value_get_int128_t$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_int128_t;
  new (return$) ::std::array<::std::uint64_t, 2>(value_get_int128_t$(value));
}

void kuzu_rs$cxxbridge1$194$value_get_internal_id(::kuzu::common::Value const &value, ::std::array<::std::uint64_t, 2> *return$) noexcept {
  ::std::array<::std::uint64_t, 2> (*value_get_internal_id$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_internal_id;
  new (return$) ::std::array<::std::uint64_t, 2>(value_get_internal_id$(value));
}

::kuzu::common::LogicalTypeID kuzu_rs$cxxbridge1$194$value_get_data_type_id(::kuzu::common::Value const &value) noexcept {
  ::kuzu::common::LogicalTypeID (*value_get_data_type_id$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_data_type_id;
  return value_get_data_type_id$(value);
}

void kuzu_rs$cxxbridge1$194$value_get_data_type(::kuzu::common::Value const &value, ::kuzu::common::LogicalType const **return$) noexcept {
  ::kuzu::common::LogicalType const &(*value_get_data_type$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_data_type;
  new (return$) ::kuzu::common::LogicalType const *(&value_get_data_type$(value));
}

::std::uint32_t kuzu_rs$cxxbridge1$194$value_get_children_size(::kuzu::common::Value const &value) noexcept {
  ::std::uint32_t (*value_get_children_size$)(::kuzu::common::Value const &) = ::kuzu_rs::value_get_children_size;
  return value_get_children_size$(value);
}

void kuzu_rs$cxxbridge1$194$value_get_child(::kuzu::common::Value const &value, ::std::uint32_t index, ::kuzu::common::Value const **return$) noexcept {
  ::kuzu::common::Value const &(*value_get_child$)(::kuzu::common::Value const &, ::std::uint32_t) = ::kuzu_rs::value_get_child;
  new (return$) ::kuzu::common::Value const *(&value_get_child$(value, index));
}

bool kuzu_rs$cxxbridge1$194$Value$isNull(::kuzu::common::Value const &self) noexcept {
  bool (::kuzu::common::Value::*isNull$)() const = &::kuzu::common::Value::isNull;
  return (self.*isNull$)();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_bool(bool value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_bool$)(bool) = ::kuzu_rs::create_value;
  return create_value_bool$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_i8(::std::int8_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_i8$)(::std::int8_t) = ::kuzu_rs::create_value;
  return create_value_i8$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_i16(::std::int16_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_i16$)(::std::int16_t) = ::kuzu_rs::create_value;
  return create_value_i16$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_i32(::std::int32_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_i32$)(::std::int32_t) = ::kuzu_rs::create_value;
  return create_value_i32$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_i64(::std::int64_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_i64$)(::std::int64_t) = ::kuzu_rs::create_value;
  return create_value_i64$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_u8(::std::uint8_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_u8$)(::std::uint8_t) = ::kuzu_rs::create_value;
  return create_value_u8$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_u16(::std::uint16_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_u16$)(::std::uint16_t) = ::kuzu_rs::create_value;
  return create_value_u16$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_u32(::std::uint32_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_u32$)(::std::uint32_t) = ::kuzu_rs::create_value;
  return create_value_u32$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_u64(::std::uint64_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_u64$)(::std::uint64_t) = ::kuzu_rs::create_value;
  return create_value_u64$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_float(float value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_float$)(float) = ::kuzu_rs::create_value;
  return create_value_float$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_double(double value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_double$)(double) = ::kuzu_rs::create_value;
  return create_value_double$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_null(::kuzu::common::LogicalType *typ) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_null$)(::std::unique_ptr<::kuzu::common::LogicalType>) = ::kuzu_rs::create_value_null;
  return create_value_null$(::std::unique_ptr<::kuzu::common::LogicalType>(typ)).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_string(::kuzu::common::LogicalTypeID typ, ::rust::Slice<::std::uint8_t const> value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_string$)(::kuzu::common::LogicalTypeID, ::rust::Slice<::std::uint8_t const>) = ::kuzu_rs::create_value_string;
  return create_value_string$(typ, value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_timestamp(::std::int64_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_timestamp$)(::std::int64_t) = ::kuzu_rs::create_value_timestamp;
  return create_value_timestamp$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_timestamp_tz(::std::int64_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_timestamp_tz$)(::std::int64_t) = ::kuzu_rs::create_value_timestamp_tz;
  return create_value_timestamp_tz$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_timestamp_ns(::std::int64_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_timestamp_ns$)(::std::int64_t) = ::kuzu_rs::create_value_timestamp_ns;
  return create_value_timestamp_ns$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_timestamp_ms(::std::int64_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_timestamp_ms$)(::std::int64_t) = ::kuzu_rs::create_value_timestamp_ms;
  return create_value_timestamp_ms$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_timestamp_sec(::std::int64_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_timestamp_sec$)(::std::int64_t) = ::kuzu_rs::create_value_timestamp_sec;
  return create_value_timestamp_sec$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_date(::std::int32_t value) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_date$)(::std::int32_t) = ::kuzu_rs::create_value_date;
  return create_value_date$(value).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_interval(::std::int32_t months, ::std::int32_t days, ::std::int64_t micros) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_interval$)(::std::int32_t, ::std::int32_t, ::std::int64_t) = ::kuzu_rs::create_value_interval;
  return create_value_interval$(months, days, micros).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_int128_t(::std::int64_t high, ::std::uint64_t low) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_int128_t$)(::std::int64_t, ::std::uint64_t) = ::kuzu_rs::create_value_int128_t;
  return create_value_int128_t$(high, low).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_uuid_t(::std::int64_t high, ::std::uint64_t low) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_uuid_t$)(::std::int64_t, ::std::uint64_t) = ::kuzu_rs::create_value_uuid_t;
  return create_value_uuid_t$(high, low).release();
}

::kuzu::common::Value *kuzu_rs$cxxbridge1$194$create_value_internal_id(::std::uint64_t offset, ::std::uint64_t table) noexcept {
  ::std::unique_ptr<::kuzu::common::Value> (*create_value_internal_id$)(::std::uint64_t, ::std::uint64_t) = ::kuzu_rs::create_value_internal_id;
  return create_value_internal_id$(offset, table).release();
}

void kuzu_rs$cxxbridge1$194$node_value_get_node_id(::kuzu::common::Value const &value, ::kuzu::common::Value const **return$) noexcept {
  ::kuzu::common::Value const &(*node_value_get_node_id$)(::kuzu::common::Value const &) = ::kuzu_rs::node_value_get_node_id;
  new (return$) ::kuzu::common::Value const *(&node_value_get_node_id$(value));
}

void kuzu_rs$cxxbridge1$194$node_value_get_label_name(::kuzu::common::Value const &value, ::rust::String *return$) noexcept {
  ::rust::String (*node_value_get_label_name$)(::kuzu::common::Value const &) = ::kuzu_rs::node_value_get_label_name;
  new (return$) ::rust::String(node_value_get_label_name$(value));
}

::std::size_t kuzu_rs$cxxbridge1$194$node_value_get_num_properties(::kuzu::common::Value const &value) noexcept {
  ::std::size_t (*node_value_get_num_properties$)(::kuzu::common::Value const &) = ::kuzu_rs::node_value_get_num_properties;
  return node_value_get_num_properties$(value);
}

void kuzu_rs$cxxbridge1$194$node_value_get_property_name(::kuzu::common::Value const &value, ::std::size_t index, ::rust::String *return$) noexcept {
  ::rust::String (*node_value_get_property_name$)(::kuzu::common::Value const &, ::std::size_t) = ::kuzu_rs::node_value_get_property_name;
  new (return$) ::rust::String(node_value_get_property_name$(value, index));
}

void kuzu_rs$cxxbridge1$194$node_value_get_property_value(::kuzu::common::Value const &value, ::std::size_t index, ::kuzu::common::Value const **return$) noexcept {
  ::kuzu::common::Value const &(*node_value_get_property_value$)(::kuzu::common::Value const &, ::std::size_t) = ::kuzu_rs::node_value_get_property_value;
  new (return$) ::kuzu::common::Value const *(&node_value_get_property_value$(value, index));
}

void kuzu_rs$cxxbridge1$194$rel_value_get_label_name(::kuzu::common::Value const &value, ::rust::String *return$) noexcept {
  ::rust::String (*rel_value_get_label_name$)(::kuzu::common::Value const &) = ::kuzu_rs::rel_value_get_label_name;
  new (return$) ::rust::String(rel_value_get_label_name$(value));
}

void kuzu_rs$cxxbridge1$194$rel_value_get_src_id(::kuzu::common::Value const &value, ::kuzu::common::Value const **return$) noexcept {
  ::kuzu::common::Value const &(*rel_value_get_src_id$)(::kuzu::common::Value const &) = ::kuzu_rs::rel_value_get_src_id;
  new (return$) ::kuzu::common::Value const *(&rel_value_get_src_id$(value));
}

void kuzu_rs$cxxbridge1$194$rel_value_get_dst_id(::kuzu::common::Value const &value, ::std::array<::std::uint64_t, 2> *return$) noexcept {
  ::std::array<::std::uint64_t, 2> (*rel_value_get_dst_id$)(::kuzu::common::Value const &) = ::kuzu_rs::rel_value_get_dst_id;
  new (return$) ::std::array<::std::uint64_t, 2>(rel_value_get_dst_id$(value));
}

::std::size_t kuzu_rs$cxxbridge1$194$rel_value_get_num_properties(::kuzu::common::Value const &value) noexcept {
  ::std::size_t (*rel_value_get_num_properties$)(::kuzu::common::Value const &) = ::kuzu_rs::rel_value_get_num_properties;
  return rel_value_get_num_properties$(value);
}

void kuzu_rs$cxxbridge1$194$rel_value_get_property_name(::kuzu::common::Value const &value, ::std::size_t index, ::rust::String *return$) noexcept {
  ::rust::String (*rel_value_get_property_name$)(::kuzu::common::Value const &, ::std::size_t) = ::kuzu_rs::rel_value_get_property_name;
  new (return$) ::rust::String(rel_value_get_property_name$(value, index));
}

void kuzu_rs$cxxbridge1$194$rel_value_get_property_value(::kuzu::common::Value const &value, ::std::size_t index, ::kuzu::common::Value const **return$) noexcept {
  ::kuzu::common::Value const &(*rel_value_get_property_value$)(::kuzu::common::Value const &, ::std::size_t) = ::kuzu_rs::rel_value_get_property_value;
  new (return$) ::kuzu::common::Value const *(&rel_value_get_property_value$(value, index));
}

void kuzu_rs$cxxbridge1$194$recursive_rel_get_nodes(::kuzu::common::Value const &value, ::kuzu::common::Value const **return$) noexcept {
  ::kuzu::common::Value const &(*recursive_rel_get_nodes$)(::kuzu::common::Value const &) = ::kuzu_rs::recursive_rel_get_nodes;
  new (return$) ::kuzu::common::Value const *(&recursive_rel_get_nodes$(value));
}

void kuzu_rs$cxxbridge1$194$recursive_rel_get_rels(::kuzu::common::Value const &value, ::kuzu::common::Value const **return$) noexcept {
  ::kuzu::common::Value const &(*recursive_rel_get_rels$)(::kuzu::common::Value const &) = ::kuzu_rs::recursive_rel_get_rels;
  new (return$) ::kuzu::common::Value const *(&recursive_rel_get_rels$(value));
}

void kuzu_rs$cxxbridge1$194$get_blob_from_bytes(::rust::Vec<::std::uint8_t> const &value, ::rust::Vec<::std::uint8_t> *return$) noexcept {
  ::rust::Vec<::std::uint8_t> (*get_blob_from_bytes$)(::rust::Vec<::std::uint8_t> const &) = ::kuzu_rs::get_blob_from_bytes;
  new (return$) ::rust::Vec<::std::uint8_t>(get_blob_from_bytes$(value));
}

::std::uint64_t kuzu_rs$cxxbridge1$194$get_storage_version() noexcept {
  ::std::uint64_t (*get_storage_version$)() = ::kuzu_rs::get_storage_version;
  return get_storage_version$();
}
} // extern "C"
} // namespace kuzu_rs

extern "C" {
static_assert(::rust::detail::is_complete<::std::remove_extent<::kuzu::common::Value>::type>::value, "definition of `::kuzu::common::Value` is required");
static_assert(sizeof(::std::unique_ptr<::kuzu::common::Value>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::kuzu::common::Value>) == alignof(void *), "");
void cxxbridge1$unique_ptr$kuzu$common$Value$null(::std::unique_ptr<::kuzu::common::Value> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::common::Value>();
}
void cxxbridge1$unique_ptr$kuzu$common$Value$raw(::std::unique_ptr<::kuzu::common::Value> *ptr, ::std::unique_ptr<::kuzu::common::Value>::pointer raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::common::Value>(raw);
}
::std::unique_ptr<::kuzu::common::Value>::element_type const *cxxbridge1$unique_ptr$kuzu$common$Value$get(::std::unique_ptr<::kuzu::common::Value> const &ptr) noexcept {
  return ptr.get();
}
::std::unique_ptr<::kuzu::common::Value>::pointer cxxbridge1$unique_ptr$kuzu$common$Value$release(::std::unique_ptr<::kuzu::common::Value> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$kuzu$common$Value$drop(::std::unique_ptr<::kuzu::common::Value> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::kuzu::common::Value>::value>{}(ptr);
}

static_assert(::rust::detail::is_complete<::std::remove_extent<::kuzu_rs::QueryParams>::type>::value, "definition of `::kuzu_rs::QueryParams` is required");
static_assert(sizeof(::std::unique_ptr<::kuzu_rs::QueryParams>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::kuzu_rs::QueryParams>) == alignof(void *), "");
void cxxbridge1$unique_ptr$kuzu_rs$QueryParams$null(::std::unique_ptr<::kuzu_rs::QueryParams> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu_rs::QueryParams>();
}
void cxxbridge1$unique_ptr$kuzu_rs$QueryParams$raw(::std::unique_ptr<::kuzu_rs::QueryParams> *ptr, ::std::unique_ptr<::kuzu_rs::QueryParams>::pointer raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu_rs::QueryParams>(raw);
}
::std::unique_ptr<::kuzu_rs::QueryParams>::element_type const *cxxbridge1$unique_ptr$kuzu_rs$QueryParams$get(::std::unique_ptr<::kuzu_rs::QueryParams> const &ptr) noexcept {
  return ptr.get();
}
::std::unique_ptr<::kuzu_rs::QueryParams>::pointer cxxbridge1$unique_ptr$kuzu_rs$QueryParams$release(::std::unique_ptr<::kuzu_rs::QueryParams> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$kuzu_rs$QueryParams$drop(::std::unique_ptr<::kuzu_rs::QueryParams> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::kuzu_rs::QueryParams>::value>{}(ptr);
}

static_assert(::rust::detail::is_complete<::std::remove_extent<::kuzu::main::Database>::type>::value, "definition of `::kuzu::main::Database` is required");
static_assert(sizeof(::std::unique_ptr<::kuzu::main::Database>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::kuzu::main::Database>) == alignof(void *), "");
void cxxbridge1$unique_ptr$kuzu$main$Database$null(::std::unique_ptr<::kuzu::main::Database> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::main::Database>();
}
void cxxbridge1$unique_ptr$kuzu$main$Database$raw(::std::unique_ptr<::kuzu::main::Database> *ptr, ::std::unique_ptr<::kuzu::main::Database>::pointer raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::main::Database>(raw);
}
::std::unique_ptr<::kuzu::main::Database>::element_type const *cxxbridge1$unique_ptr$kuzu$main$Database$get(::std::unique_ptr<::kuzu::main::Database> const &ptr) noexcept {
  return ptr.get();
}
::std::unique_ptr<::kuzu::main::Database>::pointer cxxbridge1$unique_ptr$kuzu$main$Database$release(::std::unique_ptr<::kuzu::main::Database> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$kuzu$main$Database$drop(::std::unique_ptr<::kuzu::main::Database> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::kuzu::main::Database>::value>{}(ptr);
}

static_assert(::rust::detail::is_complete<::std::remove_extent<::kuzu::main::Connection>::type>::value, "definition of `::kuzu::main::Connection` is required");
static_assert(sizeof(::std::unique_ptr<::kuzu::main::Connection>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::kuzu::main::Connection>) == alignof(void *), "");
void cxxbridge1$unique_ptr$kuzu$main$Connection$null(::std::unique_ptr<::kuzu::main::Connection> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::main::Connection>();
}
void cxxbridge1$unique_ptr$kuzu$main$Connection$raw(::std::unique_ptr<::kuzu::main::Connection> *ptr, ::std::unique_ptr<::kuzu::main::Connection>::pointer raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::main::Connection>(raw);
}
::std::unique_ptr<::kuzu::main::Connection>::element_type const *cxxbridge1$unique_ptr$kuzu$main$Connection$get(::std::unique_ptr<::kuzu::main::Connection> const &ptr) noexcept {
  return ptr.get();
}
::std::unique_ptr<::kuzu::main::Connection>::pointer cxxbridge1$unique_ptr$kuzu$main$Connection$release(::std::unique_ptr<::kuzu::main::Connection> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$kuzu$main$Connection$drop(::std::unique_ptr<::kuzu::main::Connection> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::kuzu::main::Connection>::value>{}(ptr);
}

static_assert(::rust::detail::is_complete<::std::remove_extent<::kuzu::main::PreparedStatement>::type>::value, "definition of `::kuzu::main::PreparedStatement` is required");
static_assert(sizeof(::std::unique_ptr<::kuzu::main::PreparedStatement>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::kuzu::main::PreparedStatement>) == alignof(void *), "");
void cxxbridge1$unique_ptr$kuzu$main$PreparedStatement$null(::std::unique_ptr<::kuzu::main::PreparedStatement> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::main::PreparedStatement>();
}
void cxxbridge1$unique_ptr$kuzu$main$PreparedStatement$raw(::std::unique_ptr<::kuzu::main::PreparedStatement> *ptr, ::std::unique_ptr<::kuzu::main::PreparedStatement>::pointer raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::main::PreparedStatement>(raw);
}
::std::unique_ptr<::kuzu::main::PreparedStatement>::element_type const *cxxbridge1$unique_ptr$kuzu$main$PreparedStatement$get(::std::unique_ptr<::kuzu::main::PreparedStatement> const &ptr) noexcept {
  return ptr.get();
}
::std::unique_ptr<::kuzu::main::PreparedStatement>::pointer cxxbridge1$unique_ptr$kuzu$main$PreparedStatement$release(::std::unique_ptr<::kuzu::main::PreparedStatement> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$kuzu$main$PreparedStatement$drop(::std::unique_ptr<::kuzu::main::PreparedStatement> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::kuzu::main::PreparedStatement>::value>{}(ptr);
}

static_assert(::rust::detail::is_complete<::std::remove_extent<::kuzu::main::QueryResult>::type>::value, "definition of `::kuzu::main::QueryResult` is required");
static_assert(sizeof(::std::unique_ptr<::kuzu::main::QueryResult>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::kuzu::main::QueryResult>) == alignof(void *), "");
void cxxbridge1$unique_ptr$kuzu$main$QueryResult$null(::std::unique_ptr<::kuzu::main::QueryResult> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::main::QueryResult>();
}
void cxxbridge1$unique_ptr$kuzu$main$QueryResult$raw(::std::unique_ptr<::kuzu::main::QueryResult> *ptr, ::std::unique_ptr<::kuzu::main::QueryResult>::pointer raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::main::QueryResult>(raw);
}
::std::unique_ptr<::kuzu::main::QueryResult>::element_type const *cxxbridge1$unique_ptr$kuzu$main$QueryResult$get(::std::unique_ptr<::kuzu::main::QueryResult> const &ptr) noexcept {
  return ptr.get();
}
::std::unique_ptr<::kuzu::main::QueryResult>::pointer cxxbridge1$unique_ptr$kuzu$main$QueryResult$release(::std::unique_ptr<::kuzu::main::QueryResult> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$kuzu$main$QueryResult$drop(::std::unique_ptr<::kuzu::main::QueryResult> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::kuzu::main::QueryResult>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::kuzu::processor::FlatTuple>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::kuzu::processor::FlatTuple>) == alignof(void *), "");
void cxxbridge1$shared_ptr$kuzu$processor$FlatTuple$null(::std::shared_ptr<::kuzu::processor::FlatTuple> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::kuzu::processor::FlatTuple>();
}
bool cxxbridge1$shared_ptr$kuzu$processor$FlatTuple$raw(::std::shared_ptr<::kuzu::processor::FlatTuple> *ptr, ::std::shared_ptr<::kuzu::processor::FlatTuple>::element_type *raw) noexcept {
  ::new (ptr) ::rust::shared_ptr_if_destructible<::kuzu::processor::FlatTuple>(raw);
  return ::rust::is_destructible<::kuzu::processor::FlatTuple>::value;
}
void cxxbridge1$shared_ptr$kuzu$processor$FlatTuple$clone(::std::shared_ptr<::kuzu::processor::FlatTuple> const &self, ::std::shared_ptr<::kuzu::processor::FlatTuple> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::kuzu::processor::FlatTuple>(self);
}
::std::shared_ptr<::kuzu::processor::FlatTuple>::element_type const *cxxbridge1$shared_ptr$kuzu$processor$FlatTuple$get(::std::shared_ptr<::kuzu::processor::FlatTuple> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$kuzu$processor$FlatTuple$drop(::std::shared_ptr<::kuzu::processor::FlatTuple> *self) noexcept {
  self->~shared_ptr();
}

::std::vector<::kuzu::common::LogicalType> *cxxbridge1$std$vector$kuzu$common$LogicalType$new() noexcept {
  return new ::std::vector<::kuzu::common::LogicalType>();
}
::std::size_t cxxbridge1$std$vector$kuzu$common$LogicalType$size(::std::vector<::kuzu::common::LogicalType> const &s) noexcept {
  return s.size();
}
::std::size_t cxxbridge1$std$vector$kuzu$common$LogicalType$capacity(::std::vector<::kuzu::common::LogicalType> const &s) noexcept {
  return s.capacity();
}
::kuzu::common::LogicalType *cxxbridge1$std$vector$kuzu$common$LogicalType$get_unchecked(::std::vector<::kuzu::common::LogicalType> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
bool cxxbridge1$std$vector$kuzu$common$LogicalType$reserve(::std::vector<::kuzu::common::LogicalType> *s, ::std::size_t new_cap) noexcept {
  return ::rust::if_move_constructible<::kuzu::common::LogicalType>::reserve(*s, new_cap);
}
static_assert(::rust::detail::is_complete<::std::remove_extent<::std::vector<::kuzu::common::LogicalType>>::type>::value, "definition of `::std::vector<::kuzu::common::LogicalType>` is required");
static_assert(sizeof(::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$kuzu$common$LogicalType$null(::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>>();
}
void cxxbridge1$unique_ptr$std$vector$kuzu$common$LogicalType$raw(::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>> *ptr, ::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>>::pointer raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>>(raw);
}
::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>>::element_type const *cxxbridge1$unique_ptr$std$vector$kuzu$common$LogicalType$get(::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>> const &ptr) noexcept {
  return ptr.get();
}
::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>>::pointer cxxbridge1$unique_ptr$std$vector$kuzu$common$LogicalType$release(::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$kuzu$common$LogicalType$drop(::std::unique_ptr<::std::vector<::kuzu::common::LogicalType>> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::std::vector<::kuzu::common::LogicalType>>::value>{}(ptr);
}

static_assert(::rust::detail::is_complete<::std::remove_extent<::kuzu::common::LogicalType>::type>::value, "definition of `::kuzu::common::LogicalType` is required");
static_assert(sizeof(::std::unique_ptr<::kuzu::common::LogicalType>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::kuzu::common::LogicalType>) == alignof(void *), "");
void cxxbridge1$unique_ptr$kuzu$common$LogicalType$null(::std::unique_ptr<::kuzu::common::LogicalType> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::common::LogicalType>();
}
void cxxbridge1$unique_ptr$kuzu$common$LogicalType$raw(::std::unique_ptr<::kuzu::common::LogicalType> *ptr, ::std::unique_ptr<::kuzu::common::LogicalType>::pointer raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu::common::LogicalType>(raw);
}
::std::unique_ptr<::kuzu::common::LogicalType>::element_type const *cxxbridge1$unique_ptr$kuzu$common$LogicalType$get(::std::unique_ptr<::kuzu::common::LogicalType> const &ptr) noexcept {
  return ptr.get();
}
::std::unique_ptr<::kuzu::common::LogicalType>::pointer cxxbridge1$unique_ptr$kuzu$common$LogicalType$release(::std::unique_ptr<::kuzu::common::LogicalType> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$kuzu$common$LogicalType$drop(::std::unique_ptr<::kuzu::common::LogicalType> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::kuzu::common::LogicalType>::value>{}(ptr);
}

static_assert(::rust::detail::is_complete<::std::remove_extent<::kuzu_rs::TypeListBuilder>::type>::value, "definition of `::kuzu_rs::TypeListBuilder` is required");
static_assert(sizeof(::std::unique_ptr<::kuzu_rs::TypeListBuilder>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::kuzu_rs::TypeListBuilder>) == alignof(void *), "");
void cxxbridge1$unique_ptr$kuzu_rs$TypeListBuilder$null(::std::unique_ptr<::kuzu_rs::TypeListBuilder> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu_rs::TypeListBuilder>();
}
void cxxbridge1$unique_ptr$kuzu_rs$TypeListBuilder$raw(::std::unique_ptr<::kuzu_rs::TypeListBuilder> *ptr, ::std::unique_ptr<::kuzu_rs::TypeListBuilder>::pointer raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu_rs::TypeListBuilder>(raw);
}
::std::unique_ptr<::kuzu_rs::TypeListBuilder>::element_type const *cxxbridge1$unique_ptr$kuzu_rs$TypeListBuilder$get(::std::unique_ptr<::kuzu_rs::TypeListBuilder> const &ptr) noexcept {
  return ptr.get();
}
::std::unique_ptr<::kuzu_rs::TypeListBuilder>::pointer cxxbridge1$unique_ptr$kuzu_rs$TypeListBuilder$release(::std::unique_ptr<::kuzu_rs::TypeListBuilder> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$kuzu_rs$TypeListBuilder$drop(::std::unique_ptr<::kuzu_rs::TypeListBuilder> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::kuzu_rs::TypeListBuilder>::value>{}(ptr);
}

static_assert(::rust::detail::is_complete<::std::remove_extent<::kuzu_rs::ValueListBuilder>::type>::value, "definition of `::kuzu_rs::ValueListBuilder` is required");
static_assert(sizeof(::std::unique_ptr<::kuzu_rs::ValueListBuilder>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::kuzu_rs::ValueListBuilder>) == alignof(void *), "");
void cxxbridge1$unique_ptr$kuzu_rs$ValueListBuilder$null(::std::unique_ptr<::kuzu_rs::ValueListBuilder> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu_rs::ValueListBuilder>();
}
void cxxbridge1$unique_ptr$kuzu_rs$ValueListBuilder$raw(::std::unique_ptr<::kuzu_rs::ValueListBuilder> *ptr, ::std::unique_ptr<::kuzu_rs::ValueListBuilder>::pointer raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::kuzu_rs::ValueListBuilder>(raw);
}
::std::unique_ptr<::kuzu_rs::ValueListBuilder>::element_type const *cxxbridge1$unique_ptr$kuzu_rs$ValueListBuilder$get(::std::unique_ptr<::kuzu_rs::ValueListBuilder> const &ptr) noexcept {
  return ptr.get();
}
::std::unique_ptr<::kuzu_rs::ValueListBuilder>::pointer cxxbridge1$unique_ptr$kuzu_rs$ValueListBuilder$release(::std::unique_ptr<::kuzu_rs::ValueListBuilder> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$kuzu_rs$ValueListBuilder$drop(::std::unique_ptr<::kuzu_rs::ValueListBuilder> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::kuzu_rs::ValueListBuilder>::value>{}(ptr);
}
} // extern "C"
